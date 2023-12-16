use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ConfiguredDefaults {
    pub all: bool,             // Backup all known guest systems on this host.
    pub bwlimit: i64,          // Limit I/O bandwidth (in KiB/s).
    pub compress: Compression, // Compress dump file.
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub dumpdir: Option<String>, // Store resulting files to specified directory.
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub exclude: Option<String>, // Exclude specified guest systems (assumes --all)
    #[serde(default)]
    pub exclude_path: Option<Vec<String>>, // Exclude certain files/directories (shell globs). Paths starting with '/' are anchored to the container's root,  other paths match relative to each subdirectory.
    pub ionice: i32, // Set IO priority when using the BFQ scheduler. For snapshot and suspend mode backups of VMs, this only affects the compressor. A value of 8 means the idle priority is used, otherwise the best-effort priority is used with the specified value.
    pub lockwait: i32, // Maximal time to wait for the global lock (minutes).
    #[serde(rename = "mailnotification")]
    pub mail_notification: MailNotification, // Deprecated: use 'notification-policy' instead.
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub mailto: Option<String>, // Comma-separated list of email addresses or users that should receive email notifications. Has no effect if the 'notification-target' option  is set at the same time.
    #[serde(default)]
    pub maxfiles: Option<i32>, // Deprecated: use 'prune-backups' instead. Maximal number of backup files per guest system.
    pub mode: Mode, // Backup mode.
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub node: Option<String>, // Only run if executed on this node.
    #[serde(
        default,
        rename = "notes-template",
        deserialize_with = "crate::deserializers::non_empty_str"
    )]
    pub notes_template: Option<String>, // Template string for generating notes for the backup(s). It can contain variables which will be replaced by their values. Currently supported are {{cluster}}, {{guestname}}, {{node}}, and {{vmid}}, but more might be added in the future. Needs to be a single line, newline and backslash need to be escaped as '\n' and '\\' respectively.
    #[serde(rename = "notification-policy")]
    pub notification_policy: NotificationPolicy, // Specify when to send a notification
    #[serde(
        default,
        rename = "notification-target",
        deserialize_with = "crate::deserializers::non_empty_str"
    )]
    pub notification_target: Option<String>, // Determine the target to which notifications should be sent. Can either be a notification endpoint or a notification group. This option takes precedence over 'mailto', meaning that if both are  set, the 'mailto' option will be ignored.
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub performance: Option<String>, // Other performance-related settings.
    pub pigz: i32, // Use pigz instead of gzip when N>0. N=1 uses half of cores, N>1 uses N as thread count.
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub pool: Option<String>, // Backup all known guest systems included in the specified pool.
    #[serde(default)]
    pub protected: bool, // If true, mark backup(s) as protected.
    #[serde(
        rename = "prune-backups",
        deserialize_with = "crate::deserializers::non_empty_str"
    )]
    pub prune_backus: Option<String>, // Use these retention options instead of those from the storage configuration.
    pub quiet: bool,  // Be quiet. | Spooky?
    pub remove: bool, // Prune older backups according to 'prune-backups'.
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub script: Option<String>, // Use specified hook script.
    pub stdexcludes: bool, // Exclude temporary files and logs.
    pub stop: bool,   // Stop running backup jobs on this host.
    pub stopwait: i32, // Maximal time to wait until a guest system is stopped (minutes).
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub storage: Option<String>, // Store resulting file to this storage.
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub tmpdir: Option<String>, // Store temporary files to specified directory.
    #[serde(default, deserialize_with = "crate::deserializers::non_empty_str")]
    pub vmid: Option<String>, // The ID of the guest system you want to backup.
    pub zstd: i32, // Zstd threads. N=0 uses half of the available cores, N>0 uses N as thread count.
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Compression {
    #[serde(rename = "0")]
    Zero,
    #[serde(rename = "1")]
    One,
    GZip,
    LZO,
    ZStd,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MailNotification {
    Always,
    Failure,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Snapshot,
    Suspend,
    Stop,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum NotificationPolicy {
    Always,
    Failure,
    Never,
}
