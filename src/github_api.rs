use anyhow::Result;
use colored::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Component {
    pub created_at: String,
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
    pub create_at: String,
    pub id: String,
    pub impact: String,
    pub incedent_updates: Vec<IncidentUpdate>,
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
    pub created_at: String,
    pub display_at: String,
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
    pub created_at: String,
    pub id: String,
    pub impact: String,
    pub incident_updates: Vec<IncidentUpdate>,
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
    pub fn print() -> Result<()> {
        let status = get_component_status()?;

        for component in status.components {
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

        println!("More info: {:?}", status.page.url);

        Ok(())
    }
}

#[derive(Deserialize, Debug)]
pub struct StatusInfo {
    pub page: Page,
    pub status: Status,
}

impl StatusInfo {
    pub fn print() -> Result<()> {
        let status = get_status()?;

        if status.status.indicator == "none" {
            println!("{}", status.status.description.green());
        } else if status.status.indicator == "minor" {
            println!("{}", status.status.description.yellow());
        } else if status.status.indicator == "major" {
            println!("{}", status.status.description.truecolor(255, 165, 0));
        } else if status.status.indicator == "critical" {
            println!("{}", status.status.description.red());
        } else {
            println!("{}", status.status.description);
        }

        println!();
        if let Some(updated_at) = status.page.updated_at {
            println!("Last update: {}", updated_at);
        }
        println!("More info: {}", status.page.url);

        Ok(())
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
    pub fn print() -> Result<()> {
        let summary = get_summary()?;
        if summary.status.indicator == "none" {
            println!("{}", summary.status.description.green());
        } else if summary.status.indicator == "minor" {
            println!("{}", summary.status.description.yellow());
        } else if summary.status.indicator == "major" {
            println!("{}", summary.status.description.truecolor(255, 165, 0));
        } else if summary.status.indicator == "critical" {
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

        println!();
        if let Some(updated_at) = summary.page.updated_at {
            println!("Last Updated At: {}", updated_at);
        }
        println!("More info: {}", summary.page.url);

        Ok(())
    }
}

#[derive(Deserialize, Debug)]
pub struct UnresolvedInfo {
    pub page: Page,
    pub incidents: Vec<Incident>,
}

impl UnresolvedInfo {
    pub fn print() -> Result<()> {
        let status = get_unresolved()?;

        if status.incidents.is_empty() {
            println!("No unresolved incidents reported");
            println!();
        } else {
            for incident in status.incidents {
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

                println!("    Created At: {}", incident.create_at);
                println!("    Short Link: {}", incident.shortlink);
                println!("    Status: {}", incident.status);

                if let Some(updated_at) = incident.updated_at {
                    println!("    Updated At: {}", updated_at);
                }
                println!("    Updates:");
                for update in incident.incedent_updates {
                    println!("        Update: {}", update.body);
                    println!("        create_at: {}", update.created_at);
                    println!("        status: {}", update.status);
                    if let Some(updated_at) = update.updated_at {
                        println!("        Updated At: {}", updated_at);
                    }
                }

                println!();
            }
        }

        if let Some(updated_at) = status.page.updated_at {
            println!("Last update: {}", updated_at);
        }
        println!("More info: {}", status.page.url);

        Ok(())
    }
}

fn get_component_status() -> Result<ComponentInfo> {
    let result = reqwest::blocking::get("https://www.githubstatus.com/api/v2/components.json")?
        .json::<ComponentInfo>()?;

    Ok(result)
}

fn get_status() -> Result<StatusInfo> {
    let result = reqwest::blocking::get("https://www.githubstatus.com/api/v2/status.json")?
        .json::<StatusInfo>()?;

    Ok(result)
}

fn get_summary() -> Result<SummaryInfo> {
    let result = reqwest::blocking::get("https://www.githubstatus.com/api/v2/summary.json")?
        .json::<SummaryInfo>()?;

    Ok(result)
}

fn get_unresolved() -> Result<UnresolvedInfo> {
    let result =
        reqwest::blocking::get("https://www.githubstatus.com/api/v2/incidents/unresolved.json")?
            .json::<UnresolvedInfo>()?;

    Ok(result)
}
