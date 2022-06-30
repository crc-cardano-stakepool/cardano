use crate::{
    MAINNET_MIN_FREE_DISK_SPACE_IN_GB, MAINNET_MIN_FREE_RAM_IN_GB, MAINNET_RECOMMENDED_FREE_DISK_SPACE_IN_GB, MIN_CPUS,
    MIN_CPU_FREQUENCY_IN_MHZ, RECOMMENDED_CPU_FREQUENCY_IN_MHZ, TESTNET_MIN_FREE_DISK_SPACE_IN_GB, TESTNET_MIN_FREE_RAM_IN_GB,
};
use anyhow::{anyhow, Result};
use std::path::Path;
use sysinfo::{CpuExt, DiskExt, DiskType, System, SystemExt};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SystemRequirements {
    min_cpus: u8,
    mainnet_min_free_disk_space_in_gb: u8,
    mainnet_recommended_free_disk_space_in_gb: u8,
    testnet_min_free_disk_space_in_gb: u8,
    mainnet_min_free_ram_in_gb: u8,
    testnet_min_free_ram_in_gb: u8,
    processor: SupportedCpu,
    min_processor_frequency_in_mhz: u16,
    recommended_processor_frequency_in_mhz: u16,
}

impl Default for SystemRequirements {
    fn default() -> Self {
        Self {
            min_cpus: MIN_CPUS,
            mainnet_min_free_disk_space_in_gb: MAINNET_MIN_FREE_DISK_SPACE_IN_GB,
            mainnet_recommended_free_disk_space_in_gb: MAINNET_RECOMMENDED_FREE_DISK_SPACE_IN_GB,
            testnet_min_free_disk_space_in_gb: TESTNET_MIN_FREE_DISK_SPACE_IN_GB,
            mainnet_min_free_ram_in_gb: MAINNET_MIN_FREE_RAM_IN_GB,
            testnet_min_free_ram_in_gb: TESTNET_MIN_FREE_RAM_IN_GB,
            processor: SupportedCpu::default(),
            min_processor_frequency_in_mhz: MIN_CPU_FREQUENCY_IN_MHZ,
            recommended_processor_frequency_in_mhz: RECOMMENDED_CPU_FREQUENCY_IN_MHZ,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SupportedCpu {
    intel: String,
    amd: String,
}

impl Default for SupportedCpu {
    fn default() -> Self {
        Self {
            intel: String::from("GenuineIntel"),
            amd: String::from("AuthenticAMD"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SystemInfo {
    name: String,
    disk: DiskInfo,
    memory: MemoryInfo,
    cpu: CpuInfo,
}

impl Default for SystemInfo {
    fn default() -> Self {
        let system_info = Self {
            name: SystemInfo::get_sysinfo(),
            disk: DiskInfo::default(),
            memory: MemoryInfo::default(),
            cpu: CpuInfo::default(),
        };
        log::debug!("Current System: {system_info:#?}");
        system_info
    }
}

impl SystemInfo {
    pub fn get_sysinfo() -> String {
        log::info!("Getting system info");
        if System::IS_SUPPORTED {
            log::info!("This OS is supported!");
        } else {
            log::error!("This OS isn't supported (yet?).");
            panic!()
        }
        log::info!("Getting system name");
        let mut sys = System::new_all();
        sys.refresh_all();
        let name = sys
            .name()
            .ok_or_else(|| anyhow!("Could not find determine OS distribution"))
            .unwrap();
        log::debug!("OS: {name}");
        name
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DiskInfo {
    available_space: u64,
}

impl Default for DiskInfo {
    fn default() -> Self {
        DiskInfo::get_disk_info()
    }
}

impl DiskInfo {
    pub fn get_disk_info() -> Self {
        log::info!("Getting disk info");

        let mut sys = System::new_all();
        sys.refresh_all();

        for disk in sys.disks() {
            log::info!("Found disk: {disk:#?}");
            let disk_type = disk.type_();
            log::debug!("Disk type: {:?}", disk_type);
            let file_system = disk.file_system().to_vec();
            let file_system = String::from_utf8(file_system)
                .map_err(|err| anyhow!("Failed to convert byte slice to string: {err}"))
                .unwrap();
            log::debug!("Disk filesystem: {:?}", file_system);
            let mount_point = disk.mount_point();
            log::debug!("Disk mounted at: {:?}", mount_point);
            let total_space = disk.total_space();
            log::debug!("Total disk space: {:?} B", total_space);
            let available_space = disk.available_space();
            log::debug!("Total available disk space: {:?} B", available_space);
        }
        let disk = Self {
            available_space: DiskInfo::get_available_disk_space(),
        };
        log::debug!("{disk:#?}");
        disk
    }

    pub fn get_available_disk_space() -> u64 {
        log::info!("Getting disk space");

        let mut sys = System::new_all();
        sys.refresh_all();

        let available_space: u64 = sys
            .disks()
            .iter()
            .filter(|disk| {
                let dtype = disk.type_();
                dtype == DiskType::SSD || dtype == DiskType::HDD
            })
            .filter(|disk| {
                let fs = disk.file_system().to_vec();
                let fs = String::from_utf8(fs)
                    .map_err(|err| anyhow!("Failed to convert byte slice to string: {err}"))
                    .unwrap();
                fs.eq("ext4") || fs.eq("btrfs")
            })
            .filter(|disk| {
                let mp = disk.mount_point();
                mp.eq(Path::new("/")) || mp.eq(Path::new("/home"))
            })
            .fold(0, |available_space, disk| available_space + disk.available_space());

        log::debug!("Total available useful disk space: {:?} B", available_space);
        available_space
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MemoryInfo {
    available_memory: u64,
}

impl Default for MemoryInfo {
    fn default() -> Self {
        MemoryInfo::get_memory_info()
    }
}

impl MemoryInfo {
    pub fn get_memory_info() -> Self {
        log::info!("Getting memory info");
        let memory = Self {
            available_memory: MemoryInfo::get_available_memory_in_kb(),
        };
        log::debug!("{memory:#?}");
        memory
    }
    pub fn get_available_memory_in_kb() -> u64 {
        log::info!("Getting available memory");
        let mut sys = System::new_all();
        sys.refresh_all();
        let available_memory = sys.available_memory();
        log::debug!("available memory: {} KB", available_memory);
        available_memory
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CpuInfo {
    cpu_frequency_in_mhz: u16,
    vendor: String,
    cores: u8,
}

impl Default for CpuInfo {
    fn default() -> Self {
        CpuInfo::get_cpu_info()
    }
}

impl CpuInfo {
    pub fn get_cpu_info() -> Self {
        log::info!("Getting CPU info");
        let cpu = Self {
            cpu_frequency_in_mhz: CpuInfo::get_cpu_frequency().unwrap(),
            vendor: CpuInfo::get_cpu_vendor(),
            cores: CpuInfo::get_cpu_cores(),
        };
        log::debug!("{cpu:#?}");
        cpu
    }

    pub fn get_cpu_frequency() -> Result<u16> {
        log::info!("Getting CPU frequency");
        let mut sys = System::new_all();
        sys.refresh_all();
        let brand = sys.global_cpu_info().brand();
        log::debug!("CPU brand: {:?}", brand);
        let ghz_str: Vec<&str> = brand.split_whitespace().filter(|substr| substr.contains("GHz")).collect();
        if ghz_str.len() == 1 {
            let ghz_parse = ghz_str[0]
                .replace("GHz", "")
                .replace('.', "")
                .parse::<u16>()
                .map_err(|err| anyhow!("Failed parsing {} to u16: {err}", ghz_str[0]))?;
            log::debug!("Parsed to u16: {ghz_parse}");
            let cpu_frequency_in_mhz = ghz_parse * 10;
            log::debug!("CPU MHz: {cpu_frequency_in_mhz}");
            Ok(cpu_frequency_in_mhz)
        } else {
            Err(anyhow!("Failed to determine CPU frequency"))
        }
    }

    pub fn get_cpu_vendor() -> String {
        log::info!("Getting CPU vendor");
        let mut sys = System::new_all();
        sys.refresh_all();
        let vendor = sys.global_cpu_info().vendor_id().to_string();
        log::debug!("CPU vendor: {:?}", vendor);
        vendor
    }

    pub fn get_cpu_cores() -> u8 {
        log::info!("Getting CPU cores");
        let mut sys = System::new_all();
        sys.refresh_all();
        let cores = sys
            .physical_core_count()
            .ok_or_else(|| anyhow!("Failed to determine amount of cpu cores"))
            .unwrap();
        log::debug!("Physical cores: {:?}", cores);
        cores as u8
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_all_sysinfo() {
        SystemInfo::default();
    }

    #[test]
    fn test_get_sysinfo() {
        SystemInfo::get_sysinfo();
    }

    #[test]
    fn test_get_disk_info() {
        DiskInfo::get_disk_info();
    }

    #[test]
    fn test_get_available_disk_space() {
        DiskInfo::get_available_disk_space();
    }

    #[test]
    fn test_get_memory_info() {
        MemoryInfo::get_memory_info();
    }

    #[test]
    fn test_get_available_memory_in_kb() {
        MemoryInfo::get_available_memory_in_kb();
    }

    #[test]
    fn test_get_cpu_info() {
        CpuInfo::get_cpu_info();
    }

    #[test]
    fn test_get_cpu_frequency() {
        CpuInfo::get_cpu_frequency().unwrap();
    }

    #[test]
    fn test_get_cpu_vendor() {
        CpuInfo::get_cpu_vendor();
    }

    #[test]
    fn test_get_cpu_cores() {
        CpuInfo::get_cpu_cores();
    }
}
