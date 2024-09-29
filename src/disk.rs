pub struct Partition {
    pub mount_point: String,
    total_space: u64,
    available_space: u64,
}

impl Partition {
    pub fn get_total_space_in_gb(&self) -> u64 {
        self.total_space / 1_000_000_000
    }

    pub fn get_available_space_in_gb(&self) -> u64 {
        self.available_space / 1_000_000_000
    }

    pub fn get_space_percentage(&self) -> u8 {
        let used_space = self.total_space - self.available_space;
        ((used_space as f64 / self.total_space as f64) * 100.0) as u8
    }
}

pub fn get_home_partition(partitions: &Vec<Partition>) -> Option<&Partition> {
    for partition in partitions {
        if partition.mount_point == "/home" {
            let home_partition = Some(partition);
            return home_partition;
        }
    }
    None
}

pub fn get_partition() -> Vec<Partition> {
    let mut partitions: Vec<Partition> = vec![];

    let disks = sysinfo::Disks::new_with_refreshed_list();
    for disk in disks.list() {
        let mount_point = disk.mount_point().to_str().unwrap().to_string();

        // we don't need the boot partition
        if mount_point == "/boot" {
            continue;
        }

        let disk = Partition {
            mount_point,
            total_space: disk.total_space(),
            available_space: disk.available_space(),
        };

        partitions.push(disk);
    }

    partitions
}
