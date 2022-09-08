use anyhow::Result;
use colored::*;
use serde::Deserialize;

trait GitHubApiEndpoint: Sized {
    fn get_info(url: &str) -> Result<Self>;

    fn print(&self) -> Result<()>;
}

#[derive(Deserialize, Debug)]
pub struct Component {
    pub created_at: Option<String>,
    pub description: Option<String>,
    pub id: String,
    pub name: String,
    pub page_id: String,
    pub position: u8,
    pub status: String,
    pub updated_at: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Incident {
    pub created_at: Option<String>,
    pub id: String,
    pub impact: String,
    pub incident_updates: Option<Vec<IncidentUpdate>>,
    pub monitoring_at: Option<String>,
    pub name: String,
    pub page_id: String,
    pub resolved_at: Option<String>,
    pub shortlink: String,
    pub status: String,
    pub updated_at: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct IncidentUpdate {
    pub body: String,
    pub created_at: Option<String>,
    pub display_at: Option<String>,
    pub id: String,
    pub incident_id: String,
    pub status: String,
    pub updated_at: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Page {
    pub id: String,
    pub name: String,
    pub url: String,
    pub updated_at: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ScheduledMaintenance {
    pub created_at: Option<String>,
    pub id: String,
    pub impact: String,
    pub incident_updates: Option<Vec<IncidentUpdate>>,
    pub monitoring_at: Option<String>,
    pub name: String,
    pub page_id: String,
    pub resolved_at: Option<String>,
    pub scheduled_for: String,
    pub scheduled_until: String,
    pub shortlink: String,
    pub status: String,
    pub updated_at: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Status {
    pub description: String,
    pub indicator: String,
}

#[derive(Deserialize, Debug)]
pub struct ComponentInfo {
    pub page: Page,
    pub components: Vec<Component>,
}

impl GitHubApiEndpoint for ComponentInfo {
    fn get_info(url: &str) -> Result<ComponentInfo> {
        let result = reqwest::blocking::get(url)?.json::<ComponentInfo>()?;

        Ok(result)
    }

    fn print(&self) -> Result<()> {
        for component in &self.components {
            if component.description.is_some() {
                if component.status == "operational" {
                    println!("{}: {}", component.name, component.status.green());
                } else if component.status == "degraded_performance" {
                    println!("{}: {}", component.name, component.status.yellow());
                } else if component.status == "partial_outge" {
                    println!(
                        "{}: {}",
                        component.name,
                        component.status.truecolor(255, 165, 0)
                    );
                } else if component.status == "major_outage" {
                    println!("{}: {}", component.name, component.status.red());
                } else {
                    println!("{}: {}", component.name, component.status);
                }

                if let Some(updated_at) = &component.updated_at {
                    println!("    Last Updated At: {}", updated_at);
                }

                println!();
            }
        }

        println!("More info: {:?}", self.page.url);

        Ok(())
    }
}

impl ComponentInfo {
    pub fn print_info() {
        let status = ComponentInfo::get_info("https://www.githubstatus.com/api/v2/components.json");

        match status {
            Ok(s) => ComponentInfo::print(&s).unwrap(),
            _ => println!("{}", "Error retrieving information".red()),
        };
    }
}

#[derive(Deserialize, Debug)]
pub struct MaintenanceInfo {
    pub page: Page,
    pub scheduled_maintenances: Vec<ScheduledMaintenance>,
}

impl GitHubApiEndpoint for MaintenanceInfo {
    fn get_info(url: &str) -> Result<MaintenanceInfo> {
        let result = reqwest::blocking::get(url)?.json::<MaintenanceInfo>()?;

        Ok(result)
    }

    fn print(&self) -> Result<()> {
        if self.scheduled_maintenances.is_empty() {
            println!("No unresolved incidents reported");
            println!();
        } else {
            for incident in &self.scheduled_maintenances {
                if incident.impact == "none" {
                    println!("{}", incident.name.green());
                } else if incident.impact == "minor" {
                    println!("{}", incident.name.yellow());
                } else if incident.impact == "major" {
                    println!("{}", incident.name.truecolor(255, 165, 0));
                } else if incident.impact == "critical" {
                    println!("{}", incident.name.red());
                } else {
                    println!("{}", incident.name);
                }

                if let Some(created_at) = &incident.created_at {
                    println!("    Created At: {}", created_at);
                }
                println!("    Short Link: {}", incident.shortlink);
                println!("    Status: {}", incident.status);

                if let Some(updated_at) = &incident.updated_at {
                    println!("    Updated At: {}", updated_at);
                }
                if let Some(incident_updates) = &incident.incident_updates {
                    println!("    Updates:");
                    for update in incident_updates {
                        println!("        Update: {}", update.body);
                        if let Some(created_at) = &update.created_at {
                            println!("        created_at: {}", created_at);
                        }
                        println!("        status: {}", update.status);
                        if let Some(updated_at) = &update.updated_at {
                            println!("        Updated At: {}", updated_at);
                        }
                    }
                }

                println!();
            }
        }

        if let Some(updated_at) = &self.page.updated_at {
            println!("Last update: {}", updated_at);
        }
        println!("More info: {}", self.page.url);

        Ok(())
    }
}

impl MaintenanceInfo {
    pub fn print_activate() {
        let info = MaintenanceInfo::get_info(
            "https://www.githubstatus.com/api/v2/scheduled-maintenances/active.json",
        );

        match info {
            Ok(i) => MaintenanceInfo::print(&i).unwrap(),
            _ => println!("{}", "Error retrieving information".red()),
        }
    }

    pub fn print_all() {
        let info = MaintenanceInfo::get_info(
            "https://www.githubstatus.com/api/v2/scheduled-maintenances.json",
        );

        match info {
            Ok(i) => MaintenanceInfo::print(&i).unwrap(),
            _ => println!("{}", "Error retrieving information".red()),
        }
    }

    pub fn print_upcoming() {
        let info = MaintenanceInfo::get_info(
            "https://www.githubstatus.com/api/v2/scheduled-maintenances/upcoming.json",
        );

        match info {
            Ok(i) => MaintenanceInfo::print(&i).unwrap(),
            _ => println!("{}", "Error retrieving information".red()),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct StatusInfo {
    pub page: Page,
    pub status: Status,
}

impl GitHubApiEndpoint for StatusInfo {
    fn get_info(url: &str) -> Result<StatusInfo> {
        let result = reqwest::blocking::get(url)?.json::<StatusInfo>()?;

        Ok(result)
    }

    fn print(&self) -> Result<()> {
        if self.status.indicator == "none" {
            println!("{}", self.status.description.green());
        } else if self.status.indicator == "minor" {
            println!("{}", self.status.description.yellow());
        } else if self.status.indicator == "major" {
            println!("{}", self.status.description.truecolor(255, 165, 0));
        } else if self.status.indicator == "critical" {
            println!("{}", self.status.description.red());
        } else {
            println!("{}", self.status.description);
        }

        println!();
        if let Some(updated_at) = &self.page.updated_at {
            println!("Last update: {}", updated_at);
        }
        println!("More info: {}", self.page.url);

        Ok(())
    }
}

impl StatusInfo {
    pub fn print_info() {
        let status = StatusInfo::get_info("https://www.githubstatus.com/api/v2/status.json");

        match status {
            Ok(s) => StatusInfo::print(&s).unwrap(),
            _ => println!("{}", "Error retrieving information".red()),
        };
    }
}

#[derive(Deserialize, Debug)]
pub struct SummaryInfo {
    pub page: Page,
    pub status: Status,
    pub components: Vec<Component>,
    pub incidents: Vec<Incident>,
    pub scheduled_maintenances: Vec<ScheduledMaintenance>,
}

impl GitHubApiEndpoint for SummaryInfo {
    fn get_info(url: &str) -> Result<SummaryInfo> {
        let result = reqwest::blocking::get(url)?.json::<SummaryInfo>()?;

        Ok(result)
    }

    fn print(&self) -> Result<()> {
        if self.status.indicator == "none" {
            println!("{}", self.status.description.green());
        } else if self.status.indicator == "minor" {
            println!("{}", self.status.description.yellow());
        } else if self.status.indicator == "major" {
            println!("{}", self.status.description.truecolor(255, 165, 0));
        } else if self.status.indicator == "critical" {
            println!("{}", self.status.description.red());
        } else {
            println!("{}", self.status.description);
        }

        println!();

        for component in &self.components {
            if component.description.is_some() {
                if component.status == "operational" {
                    println!("{}: {}", component.name, component.status.green());
                } else if component.status == "degraded_performance" {
                    println!("{}: {}", component.name, component.status.yellow());
                } else if component.status == "partial_outage" {
                    println!(
                        "{}: {}",
                        component.name,
                        component.status.truecolor(255, 165, 0)
                    );
                } else if component.status == "major_outage" {
                    println!("{}: {}", component.name, component.status.red());
                } else {
                }
            }
        }

        println!();
        if let Some(updated_at) = &self.page.updated_at {
            println!("Last Updated At: {}", updated_at);
        }
        println!("More info: {}", self.page.url);

        Ok(())
    }
}

impl SummaryInfo {
    pub fn print_info() {
        let summary = SummaryInfo::get_info("https://www.githubstatus.com/api/v2/summary.json");

        match summary {
            Ok(s) => SummaryInfo::print(&s).unwrap(),
            _ => println!("{}", "Error retrieving information".red()),
        };
    }
}

#[derive(Deserialize, Debug)]
pub struct IncidentInfo {
    pub page: Page,
    pub incidents: Vec<Incident>,
}

impl GitHubApiEndpoint for IncidentInfo {
    fn get_info(url: &str) -> Result<IncidentInfo> {
        let result = reqwest::blocking::get(url)?.json::<IncidentInfo>()?;

        Ok(result)
    }

    fn print(&self) -> Result<()> {
        if self.incidents.is_empty() {
            println!("No unresolved incidents reported");
            println!();
        } else {
            for incident in &self.incidents {
                if incident.impact == "none" {
                    println!("{}", incident.name.green());
                } else if incident.impact == "minor" {
                    println!("{}", incident.name.yellow());
                } else if incident.impact == "major" {
                    println!("{}", incident.name.truecolor(255, 165, 0));
                } else if incident.impact == "critical" {
                    println!("{}", incident.name.red());
                } else {
                    println!("{}", incident.name);
                }

                if let Some(created_at) = &incident.created_at {
                    println!("    Created At: {}", created_at);
                }
                println!("    Short Link: {}", incident.shortlink);
                println!("    Status: {}", incident.status);

                if let Some(updated_at) = &incident.updated_at {
                    println!("    Updated At: {}", updated_at);
                }
                if let Some(incident_updates) = &incident.incident_updates {
                    println!("    Updates:");
                    for update in incident_updates {
                        println!("        Update: {}", update.body);
                        if let Some(created_at) = &update.created_at {
                            println!("        created_at: {}", created_at);
                        }
                        println!("        status: {}", update.status);
                        if let Some(updated_at) = &update.updated_at {
                            println!("        Updated At: {}", updated_at);
                        }
                    }
                }

                println!();
            }
        }

        if let Some(updated_at) = &self.page.updated_at {
            println!("Last update: {}", updated_at);
        }
        println!("More info: {}", self.page.url);

        Ok(())
    }
}

impl IncidentInfo {
    pub fn print_all() {
        let info = IncidentInfo::get_info("https://www.githubstatus.com/api/v2/incidents.json");

        match info {
            Ok(i) => IncidentInfo::print(&i).unwrap(),
            _ => println!("{}", "Error retrieving information".red()),
        }
    }

    pub fn print_unresolved() {
        let info =
            IncidentInfo::get_info("https://www.githubstatus.com/api/v2/incidents/unresolved.json");

        match info {
            Ok(i) => IncidentInfo::print(&i).unwrap(),
            _ => println!("{}", "Error retrieving information".red()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ComponentInfo;
    use super::GitHubApiEndpoint;
    use super::IncidentInfo;
    use super::MaintenanceInfo;
    use super::StatusInfo;
    use super::SummaryInfo;

    #[test]
    fn test_print_component_info() {
        let data = r#"
            {
              "page": {
                "id": "kctbh9vrtdwd",
                "name": "GitHub",
                "url": "https://www.githubstatus.com",
                "updated_at": "2022-09-05T08:07:25Z"
              },
              "components": [
                {
                  "created_at": "2014-05-03T01:22:07.274Z",
                  "description": null,
                  "group": false,
                  "group_id": null,
                  "id": "b13yz5g2cw10",
                  "name": "API",
                  "only_show_if_degraded": false,
                  "page_id": "kctbh9vrtdwd",
                  "position": 1,
                  "showcase": true,
                  "start_date": null,
                  "status": "partial_outage",
                  "updated_at": "2014-05-14T20:34:43.340Z"
                },
                {
                  "created_at": "2014-05-03T01:22:07.286Z",
                  "description": null,
                  "group": false,
                  "group_id": null,
                  "id": "9397cnvk62zn",
                  "name": "Management Portal",
                  "only_show_if_degraded": false,
                  "page_id": "kctbh9vrtdwd",
                  "position": 2,
                  "showcase": true,
                  "start_date": null,
                  "status": "major_outage",
                  "updated_at": "2014-05-14T20:34:44.470Z"
                }
              ]
            }"#;

        let info: ComponentInfo = serde_json::from_str(data).unwrap();
        let result = info.print();
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_all_incident_info() {
        let data = r#"
            {
              "page":{
                "id":"kctbh9vrtdwd",
                "name":"GitHub",
                "url":"https://www.githubstatus.com",
                "updated_at": "2022-09-05T08:07:25Z"
              },
              "incidents": [
                {
                  "created_at": "2014-05-14T14:22:39.441-06:00",
                  "id": "cp306tmzcl0y",
                  "impact": "critical",
                  "incident_updates": [
                    {
                      "body": "Our master database has ham sandwiches flying out of the rack, and we're working our hardest to stop the bleeding. The whole site is down while we restore functionality, and we'll provide another update within 30 minutes.",
                      "created_at": "2014-05-14T14:22:40.301-06:00",
                      "display_at": "2014-05-14T14:22:40.301-06:00",
                      "id": "jdy3tw5mt5r5",
                      "incident_id": "cp306tmzcl0y",
                      "status": "identified",
                      "updated_at": "2014-05-14T14:22:40.301-06:00"
                    }
                  ],
                  "monitoring_at": null,
                  "name": "Unplanned Database Outage",
                  "page_id": "kctbh9vrtdwd",
                  "resolved_at": null,
                  "shortlink": "http://stspg.co:5000/Q0E",
                  "status": "identified",
                  "updated_at": "2014-05-14T14:35:21.711-06:00"
                },
                {
                  "created_at": "2014-05-12T14:22:39.441-06:00",
                  "id": "2z5g29qrrxvl",
                  "impact": "minor",
                  "incident_updates": [
                    {
                      "body": "A small display issue with the display of the website was discovered after a recent deploy. The deploy has been rolled back and the website is again functioning correctly.",
                      "created_at": "2014-05-12T14:22:40.301-06:00",
                      "display_at": "2014-05-12T14:22:40.301-06:00",
                      "id": "vlzc06gtjnrl",
                      "incident_id": "2z5g29qrrxvl",
                      "status": "resolved",
                      "updated_at": "2014-05-12T14:22:40.301-06:00"
                    }
                  ],
                  "monitoring_at": null,
                  "name": "Unplanned Database Outage",
                  "page_id": "kctbh9vrtdwd",
                  "resolved_at": "2014-05-12T14:22:40.301-06:00",
                  "shortlink": "http://stspg.co:5000/Q0R",
                  "status": "resolved",
                  "updated_at": "2014-05-12T14:22:40.301-06:00"
                }
              ]
            }"#;

        let info: IncidentInfo = serde_json::from_str(data).unwrap();
        let result = info.print();
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_unresolved_incidents() {
        let data = r#"
            {
              "page":{
                "id":"kctbh9vrtdwd",
                "name":"GitHub",
                "url":"https://www.githubstatus.com",
                "updated_at": "2022-09-05T08:07:25Z"
              },
              "incidents": [
                {
                  "created_at": "2014-05-14T14:22:39.441-06:00",
                  "id": "cp306tmzcl0y",
                  "impact": "critical",
                  "incident_updates": [
                    {
                      "body": "Our master database has ham sandwiches flying out of the rack, and we're working our hardest to stop the bleeding. The whole site is down while we restore functionality, and we'll provide another update within 30 minutes.",
                      "created_at": "2014-05-14T14:22:40.301-06:00",
                      "display_at": "2014-05-14T14:22:40.301-06:00",
                      "id": "jdy3tw5mt5r5",
                      "incident_id": "cp306tmzcl0y",
                      "status": "identified",
                      "updated_at": "2014-05-14T14:22:40.301-06:00"
                    }
                  ],
                  "monitoring_at": null,
                  "name": "Unplanned Database Outage",
                  "page_id": "kctbh9vrtdwd",
                  "resolved_at": null,
                  "shortlink": "http://stspg.co:5000/Q0E",
                  "status": "identified",
                  "updated_at": "2014-05-14T14:35:21.711-06:00"
                }
              ]
            }"#;

        let info: IncidentInfo = serde_json::from_str(data).unwrap();
        let result = info.print();
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_activate_maintenance() {
        let data = r#"
            {
              "page":{
                "id":"kctbh9vrtdwd",
                "name":"GitHub",
                "url":"https://www.githubstatus.com",
                "updated_at": "2022-09-05T08:07:25Z"
              },
              "scheduled_maintenances": [
                {
                  "created_at": "2014-05-14T14:27:17.303-06:00",
                  "id": "k7mf5z1gz05c",
                  "impact": "minor",
                  "incident_updates": [
                    {
                      "body": "Scheduled maintenance is currently in progress. We will provide updates as necessary.",
                      "created_at": "2014-05-14T14:34:20.036-06:00",
                      "display_at": "2014-05-14T14:34:20.036-06:00",
                      "id": "drs62w8df6fs",
                      "incident_id": "k7mf5z1gz05c",
                      "status": "in_progress",
                      "updated_at": "2014-05-14T14:34:20.036-06:00"
                    },
                    {
                      "body": "We will be performing rolling upgrades to our web tier with a new kernel version so that Heartbleed will stop making us lose sleep at night. Increased load and latency is expected, but the app should still function appropriately. We will provide updates every 30 minutes with progress of the reboots.",
                      "created_at": "2014-05-14T14:27:18.845-06:00",
                      "display_at": "2014-05-14T14:27:18.845-06:00",
                      "id": "z40y7398jqxc",
                      "incident_id": "k7mf5z1gz05c",
                      "status": "scheduled",
                      "updated_at": "2014-05-14T14:27:18.845-06:00"
                    }
                  ],
                  "monitoring_at": null,
                  "name": "Web Tier Recycle",
                  "page_id": "kctbh9vrtdwd",
                  "resolved_at": null,
                  "scheduled_for": "2014-05-14T14:30:00.000-06:00",
                  "scheduled_until": "2014-05-14T16:30:00.000-06:00",
                  "shortlink": "http://stspg.co:5000/Q0G",
                  "status": "in_progress",
                  "updated_at": "2014-05-14T14:35:12.258-06:00"
                }
              ]
            }"#;

        let info: MaintenanceInfo = serde_json::from_str(data).unwrap();
        let result = info.print();
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_all_scheduled_maintenances() {
        let data = r#"
            {
              "page":{
                "id":"kctbh9vrtdwd",
                "name":"GitHub",
                "url":"https://www.githubstatus.com",
                "updated_at": "2022-09-05T08:07:25Z"
              },
              "scheduled_maintenances": [
                {
                  "created_at": "2014-05-14T14:24:40.430-06:00",
                  "id": "w1zdr745wmfy",
                  "impact": "none",
                  "incident_updates": [
                    {
                      "body": "Our data center has informed us that they will be performing routine network maintenance. No interruption in service is expected. Any issues during this maintenance should be directed to our support center",
                      "created_at": "2014-05-14T14:24:41.913-06:00",
                      "display_at": "2014-05-14T14:24:41.913-06:00",
                      "id": "qq0vx910b3qj",
                      "incident_id": "w1zdr745wmfy",
                      "status": "scheduled",
                      "updated_at": "2014-05-14T14:24:41.913-06:00"
                    }
                  ],
                  "monitoring_at": null,
                  "name": "Network Maintenance (No Interruption Expected)",
                  "page_id": "kctbh9vrtdwd",
                  "resolved_at": null,
                  "scheduled_for": "2014-05-17T22:00:00.000-06:00",
                  "scheduled_until": "2014-05-17T23:30:00.000-06:00",
                  "shortlink": "http://stspg.co:5000/Q0F",
                  "status": "scheduled",
                  "updated_at": "2014-05-14T14:24:41.918-06:00"
                },
                {
                  "created_at": "2014-05-14T14:27:17.303-06:00",
                  "id": "k7mf5z1gz05c",
                  "impact": "minor",
                  "incident_updates": [
                    {
                      "body": "Scheduled maintenance is currently in progress. We will provide updates as necessary.",
                      "created_at": "2014-05-14T14:34:20.036-06:00",
                      "display_at": "2014-05-14T14:34:20.036-06:00",
                      "id": "drs62w8df6fs",
                      "incident_id": "k7mf5z1gz05c",
                      "status": "in_progress",
                      "updated_at": "2014-05-14T14:34:20.036-06:00"
                    },
                    {
                      "body": "We will be performing rolling upgrades to our web tier with a new kernel version so that Heartbleed will stop making us lose sleep at night. Increased load and latency is expected, but the app should still function appropriately. We will provide updates every 30 minutes with progress of the reboots.",
                      "created_at": "2014-05-14T14:27:18.845-06:00",
                      "display_at": "2014-05-14T14:27:18.845-06:00",
                      "id": "z40y7398jqxc",
                      "incident_id": "k7mf5z1gz05c",
                      "status": "scheduled",
                      "updated_at": "2014-05-14T14:27:18.845-06:00"
                    }
                  ],
                  "monitoring_at": null,
                  "name": "Web Tier Recycle",
                  "page_id": "kctbh9vrtdwd",
                  "resolved_at": null,
                  "scheduled_for": "2014-05-14T14:30:00.000-06:00",
                  "scheduled_until": "2014-05-14T16:30:00.000-06:00",
                  "shortlink": "http://stspg.co:5000/Q0G",
                  "status": "in_progress",
                  "updated_at": "2014-05-14T14:35:12.258-06:00"
                }
              ]
            }"#;

        let info: MaintenanceInfo = serde_json::from_str(data).unwrap();
        let result = info.print();
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_upcoming_maintenance() {
        let data = r#"
            {
              "page":{
                "id":"kctbh9vrtdwd",
                "name":"GitHub",
                "url":"https://www.githubstatus.com",
                "updated_at": "2022-09-05T08:07:25Z"
              },
              "scheduled_maintenances": [
                {
                  "created_at": "2014-05-14T14:24:40.430-06:00",
                  "id": "w1zdr745wmfy",
                  "impact": "none",
                  "incident_updates": [
                    {
                      "body": "Our data center has informed us that they will be performing routine network maintenance. No interruption in service is expected. Any issues during this maintenance should be directed to our support center",
                      "created_at": "2014-05-14T14:24:41.913-06:00",
                      "display_at": "2014-05-14T14:24:41.913-06:00",
                      "id": "qq0vx910b3qj",
                      "incident_id": "w1zdr745wmfy",
                      "status": "scheduled",
                      "updated_at": "2014-05-14T14:24:41.913-06:00"
                    }
                  ],
                  "monitoring_at": null,
                  "name": "Network Maintenance (No Interruption Expected)",
                  "page_id": "kctbh9vrtdwd",
                  "resolved_at": null,
                  "scheduled_for": "2014-05-17T22:00:00.000-06:00",
                  "scheduled_until": "2014-05-17T23:30:00.000-06:00",
                  "shortlink": "http://stspg.co:5000/Q0F",
                  "status": "scheduled",
                  "updated_at": "2014-05-14T14:24:41.918-06:00"
                }
              ]
            }"#;

        let info: MaintenanceInfo = serde_json::from_str(data).unwrap();
        let result = info.print();
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_status() {
        let data = r#"
            {
              "page":{
                "id":"kctbh9vrtdwd",
                "name":"GitHub",
                "url":"https://www.githubstatus.com",
                "updated_at": "2022-09-05T08:07:25Z"
              },
              "status": {
                "description": "Partial System Outage",
                "indicator": "major"
              }
            }"#;

        let info: StatusInfo = serde_json::from_str(data).unwrap();
        let result = info.print();
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_summary() {
        let data = r#"
            {
              "page": {
                "id": "kctbh9vrtdwd",
                "name": "GitHub",
                "url": "https://www.githubstatus.com",
                "updated_at": "2022-09-05T08:07:25Z"
              },
              "status": {
                "description": "Partial System Outage",
                "indicator": "major"
              },
              "components": [
                {
                  "created_at": "2014-05-03T01:22:07.274Z",
                  "description": null,
                  "id": "b13yz5g2cw10",
                  "name": "API",
                  "page_id": "kctbh9vrtdwd",
                  "position": 1,
                  "status": "partial_outage",
                  "updated_at": "2014-05-14T20:34:43.340Z"
                },
                {
                  "created_at": "2014-05-03T01:22:07.286Z",
                  "description": null,
                  "id": "9397cnvk62zn",
                  "name": "Management Portal",
                  "page_id": "kctbh9vrtdwd",
                  "position": 2,
                  "status": "major_outage",
                  "updated_at": "2014-05-14T20:34:44.470Z"
                }
              ],
              "incidents": [
                {
                  "created_at": "2014-05-14T14:22:39.441-06:00",
                  "id": "cp306tmzcl0y",
                  "impact": "critical",
                  "incident_updates": [
                    {
                      "body": "Our master database has ham sandwiches flying out of the rack, and we're working our hardest to stop the bleeding. The whole site is down while we restore functionality, and we'll provide another update within 30 minutes.",
                      "created_at": "2014-05-14T14:22:40.301-06:00",
                      "display_at": "2014-05-14T14:22:40.301-06:00",
                      "id": "jdy3tw5mt5r5",
                      "incident_id": "cp306tmzcl0y",
                      "status": "identified",
                      "updated_at": "2014-05-14T14:22:40.301-06:00"
                    }
                  ],
                  "monitoring_at": null,
                  "name": "Unplanned Database Outage",
                  "page_id": "kctbh9vrtdwd",
                  "resolved_at": null,
                  "shortlink": "http://stspg.co:5000/Q0E",
                  "status": "identified",
                  "updated_at": "2014-05-14T14:35:21.711-06:00"
                }
              ],
              "scheduled_maintenances": [
                {
                  "created_at": "2014-05-14T14:24:40.430-06:00",
                  "id": "w1zdr745wmfy",
                  "impact": "none",
                  "incident_updates": [
                    {
                      "body": "Our data center has informed us that they will be performing routine network maintenance. No interruption in service is expected. Any issues during this maintenance should be directed to our support center",
                      "created_at": "2014-05-14T14:24:41.913-06:00",
                      "display_at": "2014-05-14T14:24:41.913-06:00",
                      "id": "qq0vx910b3qj",
                      "incident_id": "w1zdr745wmfy",
                      "status": "scheduled",
                      "updated_at": "2014-05-14T14:24:41.913-06:00"
                    }
                  ],
                  "monitoring_at": null,
                  "name": "Network Maintenance (No Interruption Expected)",
                  "page_id": "kctbh9vrtdwd",
                  "resolved_at": null,
                  "scheduled_for": "2014-05-17T22:00:00.000-06:00",
                  "scheduled_until": "2014-05-17T23:30:00.000-06:00",
                  "shortlink": "http://stspg.co:5000/Q0F",
                  "status": "scheduled",
                  "updated_at": "2014-05-14T14:24:41.918-06:00"
                },
                {
                  "created_at": "2014-05-14T14:27:17.303-06:00",
                  "id": "k7mf5z1gz05c",
                  "impact": "minor",
                  "incident_updates": [
                    {
                      "body": "Scheduled maintenance is currently in progress. We will provide updates as necessary.",
                      "created_at": "2014-05-14T14:34:20.036-06:00",
                      "display_at": "2014-05-14T14:34:20.036-06:00",
                      "id": "drs62w8df6fs",
                      "incident_id": "k7mf5z1gz05c",
                      "status": "in_progress",
                      "updated_at": "2014-05-14T14:34:20.036-06:00"
                    },
                    {
                      "body": "We will be performing rolling upgrades to our web tier with a new kernel version so that Heartbleed will stop making us lose sleep at night. Increased load and latency is expected, but the app should still function appropriately. We will provide updates every 30 minutes with progress of the reboots.",
                      "created_at": "2014-05-14T14:27:18.845-06:00",
                      "display_at": "2014-05-14T14:27:18.845-06:00",
                      "id": "z40y7398jqxc",
                      "incident_id": "k7mf5z1gz05c",
                      "status": "scheduled",
                      "updated_at": "2014-05-14T14:27:18.845-06:00"
                    }
                  ],
                  "monitoring_at": null,
                  "name": "Web Tier Recycle",
                  "page_id": "kctbh9vrtdwd",
                  "resolved_at": null,
                  "scheduled_for": "2014-05-14T14:30:00.000-06:00",
                  "scheduled_until": "2014-05-14T16:30:00.000-06:00",
                  "shortlink": "http://stspg.co:5000/Q0G",
                  "status": "in_progress",
                  "updated_at": "2014-05-14T14:35:12.258-06:00"
                }
              ]
            }"#;

        let info: SummaryInfo = serde_json::from_str(data).unwrap();
        let result = info.print();
        assert!(result.is_ok());
    }
}
