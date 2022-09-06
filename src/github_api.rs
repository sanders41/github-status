use anyhow::Result;
use colored::*;
use serde::Deserialize;

trait GitHubApiEndpoint: Sized {
    fn get_info(url: &str) -> Result<Self>;
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
}

impl ComponentInfo {
    pub fn print() {
        let status = ComponentInfo::get_info("https://www.githubstatus.com/api/v2/components.json");

        match status {
            Ok(s) => {
                for component in s.components {
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

                        if let Some(updated_at) = component.updated_at {
                            println!("    Last Updated At: {}", updated_at);
                        }

                        println!();
                    }
                }

                println!("More info: {:?}", s.page.url);
            }
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
}

impl MaintenanceInfo {
    fn print(self) {
        if self.scheduled_maintenances.is_empty() {
            println!("No unresolved incidents reported");
            println!();
        } else {
            for incident in self.scheduled_maintenances {
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

                if let Some(created_at) = incident.created_at {
                    println!("    Created At: {}", created_at);
                }
                println!("    Short Link: {}", incident.shortlink);
                println!("    Status: {}", incident.status);

                if let Some(updated_at) = incident.updated_at {
                    println!("    Updated At: {}", updated_at);
                }
                if let Some(incident_updates) = incident.incident_updates {
                    println!("    Updates:");
                    for update in incident_updates {
                        println!("        Update: {}", update.body);
                        if let Some(created_at) = update.created_at {
                            println!("        created_at: {}", created_at);
                        }
                        println!("        status: {}", update.status);
                        if let Some(updated_at) = update.updated_at {
                            println!("        Updated At: {}", updated_at);
                        }
                    }
                }

                println!();
            }
        }

        if let Some(updated_at) = self.page.updated_at {
            println!("Last update: {}", updated_at);
        }
        println!("More info: {}", self.page.url);
    }

    pub fn print_activate() {
        let info = MaintenanceInfo::get_info(
            "https://www.githubstatus.com/api/v2/scheduled-maintenances/active.json",
        );

        match info {
            Ok(i) => MaintenanceInfo::print(i),
            _ => println!("{}", "Error retrieving information".red()),
        }
    }

    pub fn print_all() {
        let info = MaintenanceInfo::get_info(
            "https://www.githubstatus.com/api/v2/scheduled-maintenances.json",
        );

        match info {
            Ok(i) => MaintenanceInfo::print(i),
            _ => println!("{}", "Error retrieving information".red()),
        }
    }

    pub fn print_upcoming() {
        let info = MaintenanceInfo::get_info(
            "https://www.githubstatus.com/api/v2/scheduled-maintenances/upcoming.json",
        );

        match info {
            Ok(i) => MaintenanceInfo::print(i),
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
}

impl StatusInfo {
    pub fn print() {
        let status = StatusInfo::get_info("https://www.githubstatus.com/api/v2/status.json");

        match status {
            Ok(s) => {
                if s.status.indicator == "none" {
                    println!("{}", s.status.description.green());
                } else if s.status.indicator == "minor" {
                    println!("{}", s.status.description.yellow());
                } else if s.status.indicator == "major" {
                    println!("{}", s.status.description.truecolor(255, 165, 0));
                } else if s.status.indicator == "critical" {
                    println!("{}", s.status.description.red());
                } else {
                    println!("{}", s.status.description);
                }

                println!();
                if let Some(updated_at) = s.page.updated_at {
                    println!("Last update: {}", updated_at);
                }
                println!("More info: {}", s.page.url);
            }
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
}

impl SummaryInfo {
    pub fn print() {
        let summary = SummaryInfo::get_info("https://www.githubstatus.com/api/v2/summary.json");

        match summary {
            Ok(s) => {
                if s.status.indicator == "none" {
                    println!("{}", s.status.description.green());
                } else if s.status.indicator == "minor" {
                    println!("{}", s.status.description.yellow());
                } else if s.status.indicator == "major" {
                    println!("{}", s.status.description.truecolor(255, 165, 0));
                } else if s.status.indicator == "critical" {
                    println!("{}", s.status.description.red());
                } else {
                    println!("{}", s.status.description);
                }

                println!();

                for component in s.components {
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
                if let Some(updated_at) = s.page.updated_at {
                    println!("Last Updated At: {}", updated_at);
                }
                println!("More info: {}", s.page.url);
            }
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
}

impl IncidentInfo {
    fn print(self) {
        if self.incidents.is_empty() {
            println!("No unresolved incidents reported");
            println!();
        } else {
            for incident in self.incidents {
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

                if let Some(created_at) = incident.created_at {
                    println!("    Created At: {}", created_at);
                }
                println!("    Short Link: {}", incident.shortlink);
                println!("    Status: {}", incident.status);

                if let Some(updated_at) = incident.updated_at {
                    println!("    Updated At: {}", updated_at);
                }
                if let Some(incident_updates) = incident.incident_updates {
                    println!("    Updates:");
                    for update in incident_updates {
                        println!("        Update: {}", update.body);
                        if let Some(created_at) = update.created_at {
                            println!("        created_at: {}", created_at);
                        }
                        println!("        status: {}", update.status);
                        if let Some(updated_at) = update.updated_at {
                            println!("        Updated At: {}", updated_at);
                        }
                    }
                }

                println!();
            }
        }

        if let Some(updated_at) = self.page.updated_at {
            println!("Last update: {}", updated_at);
        }
        println!("More info: {}", self.page.url);
    }

    pub fn print_all() {
        let info = IncidentInfo::get_info("https://www.githubstatus.com/api/v2/incidents.json");

        match info {
            Ok(i) => IncidentInfo::print(i),
            _ => println!("{}", "Error retrieving information".red()),
        }
    }

    pub fn print_unresolved() {
        let info =
            IncidentInfo::get_info("https://www.githubstatus.com/api/v2/incidents/unresolved.json");

        match info {
            Ok(i) => IncidentInfo::print(i),
            _ => println!("{}", "Error retrieving information".red()),
        }
    }
}
