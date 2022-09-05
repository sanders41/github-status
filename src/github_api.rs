use anyhow::Result;
use colored::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Component {
    pub created_at: String, // TODO: change to date time
    pub description: Option<String>,
    pub id: String,
    pub name: String,
    pub page_id: String,
    pub position: u8,
    pub status: String,
    pub updated_at: Option<String>, // TODO: change to date time
}

#[derive(Deserialize, Debug)]
pub struct Incident {
    pub create_at: String, // TODO: change to date time
    pub id: String,
    pub impact: String,
    pub incedent_updates: Vec<IncidentUpdate>,
    pub monitoring_at: Option<String>, // TODO: change to date time
    pub name: String,
    pub page_id: String,
    pub resolved_at: Option<String>, // TODO: change to date time
    pub shortlink: String,
    pub status: String,
    pub updated_at: Option<String>, // TODO: change to date time
}

#[derive(Deserialize, Debug)]
pub struct IncidentUpdate {
    pub body: String,
    pub created_at: String, // TODO: change to date time
    pub display_at: String, // TODO: change to date time
    pub id: String,
    pub incident_id: String,
    pub status: String,
    pub updated_at: Option<String>, // TODO: change to date time
}

#[derive(Deserialize, Debug)]
pub struct Page {
    pub id: String,
    pub name: String,
    pub url: String,
    pub update_at: Option<String>, // TODO: change to date time
}

#[derive(Deserialize, Debug)]
pub struct ScheduledMaintenance {
    pub created_at: String, // TODO: change to date time
    pub id: String,
    pub impact: String,
    pub incident_updates: Vec<IncidentUpdate>,
    pub monitoring_at: Option<String>, // TODO: change to date time
    pub name: String,
    pub page_id: String,
    pub resolved_at: Option<String>, // TODO: change to date time
    pub scheduled_for: String,       // TODO: change to date time
    pub scheduled_until: String,     // TODO: change to date time
    pub shortlink: String,
    pub status: String,
    pub updated_at: Option<String>, // TODO: change to date time
}

#[derive(Deserialize, Debug)]
pub struct Status {
    pub description: String,
    pub indicator: String,
}

#[derive(Deserialize, Debug)]
pub struct Summary {
    pub page: Page,
    pub status: Status,
    pub components: Vec<Component>,
    pub incidents: Vec<Incident>,
    pub scheduled_maintenances: Vec<ScheduledMaintenance>,
}

impl Summary {
    pub fn print_summary() -> Result<()> {
        let summary = get_summary()?;
        if summary.status.description == "All Systems Operational" {
            println!("{}", summary.status.description.green());
        } else if summary.status.description == "Partial System Outage" {
            println!("{}", summary.status.description.yellow());
        } else if summary.status.description == "Major Service Outage" {
            println!("{}", summary.status.description.red());
        } else {
            println!("{}", summary.status.description);
        }

        println!();

        for component in summary.components {
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

        Ok(())
    }
}

fn get_summary() -> Result<Summary> {
    let result = reqwest::blocking::get("https://www.githubstatus.com/api/v2/summary.json")?
        .json::<Summary>()?;

    Ok(result)
}
