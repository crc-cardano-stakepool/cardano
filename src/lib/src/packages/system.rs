use crate::{
    MAINNET_MIN_FREE_DISK_SPACE_IN_GB, MAINNET_MIN_FREE_RAM_IN_GB, MAINNET_RECOMMENDED_FREE_DISK_SPACE_IN_GB, MIN_CORES,
    MIN_CPU_FREQUENCY_IN_MHZ, RECOMMENDED_CPU_FREQUENCY_IN_MHZ, TESTNET_MIN_FREE_DISK_SPACE_IN_GB, TESTNET_MIN_FREE_RAM_IN_GB,
};
use anyhow::anyhow;
use sysinfo::{CpuExt, DiskExt, System, SystemExt};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SystemRequirements {
    pub min_cores: u8,
    pub mainnet_min_free_disk_space_in_gb: u8,
    pub mainnet_recommended_free_disk_space_in_gb: u8,
    pub testnet_min_free_disk_space_in_gb: u8,
    pub mainnet_min_free_ram_in_gb: u8,
    pub testnet_min_free_ram_in_gb: u8,
    pub processor: SupportedCpu,
    pub min_processor_frequency_in_mhz: u16,
    pub recommended_processor_frequency_in_mhz: u16,
}

impl Default for SystemRequirements {
    fn default() -> Self {
        Self {
            min_cores: MIN_CORES,
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

impl SystemRequirements {
    pub fn check_requirements() -> bool {
        log::debug!("Checking system requirements");
        let system = SystemRequirements::default();
        let current = SystemInfo::default();
        let os_ok = SystemRequirements::check_os(&system, current.name);
        let cpu_ok = SystemRequirements::check_cpu(&system, current.cpu);
        let disk_ok = SystemRequirements::check_disk(&system, current.disk);
        let memory_ok = SystemRequirements::check_memory(&system, current.memory);
        if os_ok && cpu_ok && disk_ok && memory_ok {
            log::info!("System meets all the requirements to run a cardano-node!");
            return true;
        }
        log::error!("System doesn't meet all the requirements to run a cardano-node");
        false
    }
    pub fn check_os(&self, name: String) -> bool {
        log::debug!("Checking OS");
        match name.as_str() {
            "Ubuntu" => {
                log::debug!("{name} is supported!");
                true
            }
            "Debian" => {
                log::debug!("{name} is supported!");
                true
            }
            "Linux Mint" => {
                log::debug!("{name} is supported!");
                true
            }
            "Red Hat" => {
                log::debug!("{name} is supported!");
                true
            }
            "Fedora" => {
                log::debug!("{name} is supported!");
                true
            }
            "CentOs" => {
                log::debug!("{name} is supported!");
                true
            }
            _ => {
                log::error!("{name} is not supported");
                false
            }
        }
    }
    pub fn check_cpu(&self, cpu: CpuInfo) -> bool {
        log::debug!("Checking CPU");
        let cores_ok = self.check_cpu_cores(cpu.cores);
        let vendor_ok = self.check_cpu_vendor(cpu.vendor);
        let frequency_ok = self.check_cpu_frequency(cpu.cpu_frequency_in_mhz);
        if cores_ok && vendor_ok && frequency_ok {
            log::debug!("CPU meets the requirements to run a cardano node");
            return true;
        }
        log::error!("CPU doesn't meet the requirements to run a cardano node");
        false
    }
    pub fn check_cpu_cores(&self, cores: u8) -> bool {
        log::debug!("Checking CPU cores");
        if cores >= self.min_cores {
            log::debug!("CPU has enough cores to run a cardano node");
            return true;
        }
        log::error!("CPU doesn't have enough cores to run a cardano node");
        log::error!("At least {} cores are required", self.min_cores);
        false
    }
    pub fn check_cpu_vendor(&self, vendor: String) -> bool {
        log::debug!("Checking CPU vendor");
        let supported_vendor = SupportedCpu::default();
        if vendor.eq(&supported_vendor.intel) || vendor.eq(&supported_vendor.amd) {
            log::debug!("CPU vendor is supported");
            return true;
        }
        log::error!("CPU vendor isn't supported");
        log::error!("Only Intel or AMD processors are supported");
        false
    }
    pub fn check_cpu_frequency(&self, frequency: u16) -> bool {
        log::debug!("Checking CPU frequency");
        if frequency >= self.recommended_processor_frequency_in_mhz {
            log::debug!("CPU meets the recommended frequency requirements for stake pools");
            return true;
        }
        if frequency >= self.min_processor_frequency_in_mhz {
            log::debug!("CPU meets the minimal frequency requirements");
            return true;
        }
        log::error!("CPU doesn't meet the requirements to run a cardano node");
        log::error!("At least 1.6GHz is required");
        false
    }
    pub fn check_disk(&self, disk: DiskInfo) -> bool {
        log::debug!("Checking disk");
        const GB_TO_B_CONVERSION_RATIO: u64 = 1073741824;
        let test_net_min_free_disk_space_in_b = self.testnet_min_free_disk_space_in_gb as u64 * GB_TO_B_CONVERSION_RATIO;
        let mainnet_min_free_disk_space_in_b = self.mainnet_min_free_disk_space_in_gb as u64 * GB_TO_B_CONVERSION_RATIO;
        let mainnet_recommended_min_free_disk_space_in_b = self.mainnet_recommended_free_disk_space_in_gb as u64 * GB_TO_B_CONVERSION_RATIO;
        if disk.available_space_in_b >= mainnet_recommended_min_free_disk_space_in_b {
            log::debug!("Disk has enough space to run a cardano node in mainnet for future growth");
            return true;
        }
        if disk.available_space_in_b >= mainnet_min_free_disk_space_in_b {
            log::debug!("Disk has enough space to run a cardano node in mainnet");
            return true;
        }
        if disk.available_space_in_b >= test_net_min_free_disk_space_in_b {
            log::debug!("Disk has enough space to run a cardano node in testnet");
            return true;
        }
        log::error!("Disk does not have enough space to run a cardano node");
        log::error!("At least {TESTNET_MIN_FREE_DISK_SPACE_IN_GB} GB are required to run a node in testnet");
        log::error!("At least {MAINNET_MIN_FREE_DISK_SPACE_IN_GB} GB are required to run a node in mainnet");
        log::error!("At least {MAINNET_RECOMMENDED_FREE_DISK_SPACE_IN_GB} GB are required to run a future proof node in mainnet");
        false
    }
    pub fn check_memory(&self, memory: MemoryInfo) -> bool {
        log::debug!("Checking RAM");
        const GB_TO_KB_CONVERSION_RATIO: u64 = 1048576;
        let testnet_free_ram_in_kb = self.testnet_min_free_ram_in_gb as u64 * GB_TO_KB_CONVERSION_RATIO;
        let mainnet_free_ram_in_kb = self.mainnet_min_free_ram_in_gb as u64 * GB_TO_KB_CONVERSION_RATIO;
        if memory.available_memory_in_kb >= mainnet_free_ram_in_kb {
            log::debug!("System has enough RAM to run a cardano node in mainnet");
            return true;
        }
        if memory.available_memory_in_kb >= testnet_free_ram_in_kb {
            log::debug!("System has enough RAM to run a cardano node in testnet");
            return true;
        }
        log::error!("System does not have enough RAM to run a cardano node");
        log::error!("At least {TESTNET_MIN_FREE_RAM_IN_GB} GB of RAM are required to run a cardano node in testnet");
        log::error!("At least {MAINNET_MIN_FREE_RAM_IN_GB} GB of RAM are required to run a cardano node in mainnet");
        false
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SupportedCpu {
    pub intel: String,
    pub amd: String,
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
    pub name: String,
    pub disk: DiskInfo,
    pub memory: MemoryInfo,
    pub cpu: CpuInfo,
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
        if !System::IS_SUPPORTED {
            log::error!("This OS isn't supported (yet?).");
            panic!("Can't determine anything about the running system");
        }
        log::debug!("Getting system name");
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
    pub available_space_in_b: u64,
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
            available_space_in_b: DiskInfo::get_available_disk_space(),
        };
        log::debug!("{disk:#?}");
        disk
    }
    pub fn get_available_disk_space() -> u64 {
        log::debug!("Getting disk space");
        let mut sys = System::new_all();
        sys.refresh_all();
        let available_space: u64 = sys
            .disks()
            .iter()
            .filter(|disk| {
                let fs = disk.file_system().to_vec();
                let fs = String::from_utf8(fs)
                    .map_err(|err| anyhow!("Failed to convert byte slice to string: {err}"))
                    .unwrap();
                fs.eq("ext4") || fs.eq("btrfs")
            })
            .fold(0, |available_space, disk| available_space + disk.available_space());
        log::debug!("Total available useful disk space: {:?} B", available_space);
        available_space
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MemoryInfo {
    pub available_memory_in_kb: u64,
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
            available_memory_in_kb: MemoryInfo::get_available_memory_in_kb(),
        };
        log::debug!("{memory:#?}");
        memory
    }
    pub fn get_available_memory_in_kb() -> u64 {
        log::debug!("Getting available memory");
        let mut sys = System::new_all();
        sys.refresh_all();
        let available_memory = sys.available_memory();
        log::debug!("available memory: {} KB", available_memory);
        available_memory
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CpuInfo {
    pub cpu_frequency_in_mhz: u16,
    pub vendor: String,
    pub cores: u8,
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
            cpu_frequency_in_mhz: CpuInfo::get_cpu_frequency(),
            vendor: CpuInfo::get_cpu_vendor(),
            cores: CpuInfo::get_cpu_cores(),
        };
        log::debug!("{cpu:#?}");
        cpu
    }
    pub fn get_cpu_frequency() -> u16 {
        log::debug!("Getting CPU frequency");
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
                .map_err(|err| anyhow!("Failed parsing {} to u16: {err}", ghz_str[0]))
                .unwrap();
            log::debug!("Parsed to u16: {ghz_parse}");
            let cpu_frequency_in_mhz = ghz_parse * 10;
            log::debug!("CPU MHz: {cpu_frequency_in_mhz}");
            cpu_frequency_in_mhz
        } else {
            log::error!("Failed to determine CPU frequency");
            0
        }
    }
    pub fn get_cpu_vendor() -> String {
        log::debug!("Getting CPU vendor");
        let mut sys = System::new_all();
        sys.refresh_all();
        let vendor = sys.global_cpu_info().vendor_id().to_string();
        log::debug!("CPU vendor: {:?}", vendor);
        vendor
    }
    pub fn get_cpu_cores() -> u8 {
        log::debug!("Getting CPU cores");
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
    fn test_check_requirements() {
        assert!(SystemRequirements::check_requirements())
    }

    #[test]
    fn test_check_os() {
        let system = SystemRequirements::default();
        assert_eq!(system.check_os("Ubuntu".to_string()), true);
        assert_eq!(system.check_os("Debian".to_string()), true);
        assert_eq!(system.check_os("Arch Linux".to_string()), false);
    }

    #[test]
    fn test_check_cpu() {
        let system = SystemRequirements::default();
        let cpu = CpuInfo {
            cpu_frequency_in_mhz: 2000,
            vendor: "GenuineIntel".to_string(),
            cores: 2,
        };
        assert_eq!(system.check_cpu(cpu), true);
        let cpu = CpuInfo {
            cpu_frequency_in_mhz: 1599,
            vendor: "GenuineIntel".to_string(),
            cores: 2,
        };
        assert_eq!(system.check_cpu(cpu), false);
        let cpu = CpuInfo {
            cpu_frequency_in_mhz: 1600,
            vendor: "AuthenticAMD".to_string(),
            cores: 2,
        };
        assert_eq!(system.check_cpu(cpu), true);
        let cpu = CpuInfo {
            cpu_frequency_in_mhz: 1600,
            vendor: "AuthenticAMD".to_string(),
            cores: 1,
        };
        assert_eq!(system.check_cpu(cpu), false);
        let cpu = CpuInfo {
            cpu_frequency_in_mhz: 1600,
            vendor: "Unknown CPU Vendor".to_string(),
            cores: 1,
        };
        assert_eq!(system.check_cpu(cpu), false);
    }

    #[test]
    fn test_check_cpu_cores() {
        let system = SystemRequirements::default();
        assert_eq!(system.check_cpu_cores(2), true);
        assert_eq!(system.check_cpu_cores(1), false);
    }

    #[test]
    fn test_check_cpu_vendors() {
        let system = SystemRequirements::default();
        assert_eq!(system.check_cpu_vendor("GenuineIntel".to_string()), true);
        assert_eq!(system.check_cpu_vendor("AuthenticAMD".to_string()), true);
        assert_eq!(system.check_cpu_vendor("Anything else".to_string()), false);
    }

    #[test]
    fn test_check_cpu_frequency() {
        let system = SystemRequirements::default();
        assert_eq!(system.check_cpu_frequency(2000), true);
        assert_eq!(system.check_cpu_frequency(1600), true);
        assert_eq!(system.check_cpu_frequency(1599), false);
    }

    #[test]
    fn test_check_disk() {
        let system = SystemRequirements::default();
        let _20_gb_in_b = 21474836480;
        let disk = DiskInfo {
            available_space_in_b: _20_gb_in_b,
        };
        assert_eq!(system.check_disk(disk), true);
        let disk = DiskInfo {
            available_space_in_b: _20_gb_in_b - 1,
        };
        assert_eq!(system.check_disk(disk), false);
        let disk = DiskInfo {
            available_space_in_b: _20_gb_in_b * 4,
        };
        assert_eq!(system.check_disk(disk), true);
        let disk = DiskInfo {
            available_space_in_b: _20_gb_in_b * 5,
        };
        assert_eq!(system.check_disk(disk), true);
    }

    #[test]
    fn test_check_memory() {
        let system = SystemRequirements::default();
        let _4_gb_in_kb: u64 = 4194304;
        let memory = MemoryInfo {
            available_memory_in_kb: _4_gb_in_kb,
        };
        assert_eq!(system.check_memory(memory), true);
        let memory = MemoryInfo {
            available_memory_in_kb: _4_gb_in_kb - 1,
        };
        assert_eq!(system.check_memory(memory), false);
        let memory = MemoryInfo {
            available_memory_in_kb: _4_gb_in_kb * 4,
        };
        assert_eq!(system.check_memory(memory), true);
    }

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
        CpuInfo::get_cpu_frequency();
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
