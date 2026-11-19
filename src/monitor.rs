use std::{thread, time::Duration};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use systemstat::{Memory, Platform, System};

#[derive(Serialize, Deserialize)]
pub struct MachineMount {
    pub mounted_from: String,
    pub mounted_on: String,
    pub total_space: u64,
    pub available_space: u64,
}

#[derive(Serialize, Deserialize)]
pub struct MachineStatistics {
    pub average_cpu_usage: f32,
    pub total_memory: u64,
    pub free_memory: u64,
    pub mounts: Vec<MachineMount>
}

pub fn get_machine_info() {
    
}

pub async fn get_cpu_usage(sys: &System) -> Result<f32> {
    let cpu_load_raw = sys.cpu_load_aggregate()?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let cpu_load = cpu_load_raw.done()?;
    let average_cpu_usage = (cpu_load.interrupt + cpu_load.user + cpu_load.system + cpu_load.nice)*100.0;
    Ok(average_cpu_usage)
}

pub async fn get_machine_statistics() -> Result<MachineStatistics> {
    let sys = System::new();
    let mut machine_statistics: MachineStatistics = MachineStatistics {
        average_cpu_usage: 0.0,
        total_memory: 0,
        free_memory: 0,
        mounts: Vec::new()
    };
    machine_statistics.average_cpu_usage = get_cpu_usage(&sys).await?;
    let memory = sys.memory()?;
    machine_statistics.total_memory = memory.total.as_u64();
    machine_statistics.free_memory = memory.free.as_u64();
    let mounts = sys.mounts()?;
    for mount in mounts {
        if (mount.total.as_u64() != 0) {
            let mount_item = MachineMount {
                mounted_from: mount.fs_mounted_from,
                mounted_on: mount.fs_mounted_on,
                total_space: mount.total.as_u64(),
                available_space: mount.avail.as_u64(),
            };
            machine_statistics.mounts.insert(machine_statistics.mounts.len(), mount_item);
        }
    }
    Ok(machine_statistics)
}