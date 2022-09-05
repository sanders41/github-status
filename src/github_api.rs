use anyhow::Result;
use colored::*;
use serde::Deserialize;

enum IncidentType {
    All,
    Unresolved,
}

enum MaintenanceType {
    Active,
    All,
    Upcoming,
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

impl ComponentInfo {
    fn get_component_status() -> Result<ComponentInfo> {
        let result = reqwest::blocking::get("https://www.githubstatus.com/api/v2/components.json")?
            .json::<ComponentInfo>()?;

        Ok(result)
    }

    pub fn print() {
        let status = ComponentInfo::get_component_status();

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

impl MaintenanceInfo {
    fn get_maintenance(maintenance_type: MaintenanceType) -> Result<MaintenanceInfo> {
        let result: MaintenanceInfo = match maintenance_type {
            MaintenanceType::Active => reqwest::blocking::get(
                "https://www.githubstatus.com/api/v2/scheduled-maintenances/active.json",
            )?
            .json::<MaintenanceInfo>()?,
            MaintenanceType::All => reqwest::blocking::get(
                "https://www.githubstatus.com/api/v2/scheduled-maintenances.json",
            )?
            .json::<MaintenanceInfo>()?,
            MaintenanceType::Upcoming => reqwest::blocking::get(
                "https://www.githubstatus.com/api/v2/scheduled-maintenances/upcoming.json",
            )?
            .json::<MaintenanceInfo>()?,
        };

        Ok(result)
    }

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
        let info = MaintenanceInfo::get_maintenance(MaintenanceType::Active);

        match info {
            Ok(i) => MaintenanceInfo::print(i),
            _ => println!("{}", "Error retrieving information".red()),
        }
    }

    pub fn print_all() {
        let info = MaintenanceInfo::get_maintenance(MaintenanceType::All);

        match info {
            Ok(i) => MaintenanceInfo::print(i),
            _ => println!("{}", "Error retrieving information".red()),
        }
    }

    pub fn print_upcoming() {
        let info = MaintenanceInfo::get_maintenance(MaintenanceType::Upcoming);

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

impl StatusInfo {
    fn get_status() -> Result<StatusInfo> {
        let result = reqwest::blocking::get("https://www.githubstatus.com/api/v2/status.json")?
            .json::<StatusInfo>()?;

        Ok(result)
    }

    pub fn print() {
        let status = StatusInfo::get_status();

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

impl SummaryInfo {
    fn get_summary() -> Result<SummaryInfo> {
        let result = reqwest::blocking::get("https://www.githubstatus.com/api/v2/summary.json")?
            .json::<SummaryInfo>()?;

        Ok(result)
    }

    pub fn print() {
        let summary = SummaryInfo::get_summary();

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

impl IncidentInfo {
    fn get_incidents(incident_type: IncidentType) -> Result<IncidentInfo> {
        let result: IncidentInfo = match incident_type {
            IncidentType::All => {
                reqwest::blocking::get("https://www.githubstatus.com/api/v2/incidents.json")?
                    .json::<IncidentInfo>()?
            }
            IncidentType::Unresolved => reqwest::blocking::get(
                "https://www.githubstatus.com/api/v2/incidents/unresolved.json",
            )?
            .json::<IncidentInfo>()?,
        };

        Ok(result)
    }

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
        let info = IncidentInfo::get_incidents(IncidentType::All);

        match info {
            Ok(i) => IncidentInfo::print(i),
            _ => println!("{}", "Error retrieving information".red()),
        }
    }

    pub fn print_unresolved() {
        let info = IncidentInfo::get_incidents(IncidentType::Unresolved);

        match info {
            Ok(i) => IncidentInfo::print(i),
            _ => println!("{}", "Error retrieving information".red()),
        }
    }
}
