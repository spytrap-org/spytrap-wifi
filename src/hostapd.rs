use crate::errors::*;
use rand::Rng;
use tokio::process::Command;
use tokio::fs;

// TODO: this should just pick a random word
pub fn pwgen() -> String {
    /*
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    */
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const PASSWORD_LEN: usize = 10;
    let mut rng = rand::thread_rng();

    (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

// TODO: all arguments are unsafe
fn mkconfig(interface: &str, ssid: &str, password: &str) -> String {
	format!("
interface={}

logger_syslog=-1
logger_syslog_level=2
logger_stdout=-1
logger_stdout_level=2

ctrl_interface=/run/hostapd
ctrl_interface_group=0

ssid={}
country_code=DE
hw_mode=g
channel=11
beacon_int=100
dtim_period=2
max_num_sta=255
rts_threshold=-1
fragm_threshold=-1
macaddr_acl=0
auth_algs=3
ignore_broadcast_ssid=0

wmm_enabled=1
wmm_ac_bk_cwmin=4
wmm_ac_bk_cwmax=10
wmm_ac_bk_aifs=7
wmm_ac_bk_txop_limit=0
wmm_ac_bk_acm=0
wmm_ac_be_aifs=3
wmm_ac_be_cwmin=4
wmm_ac_be_cwmax=10
wmm_ac_be_txop_limit=0
wmm_ac_be_acm=0
wmm_ac_vi_aifs=2
wmm_ac_vi_cwmin=3
wmm_ac_vi_cwmax=4
wmm_ac_vi_txop_limit=94
wmm_ac_vi_acm=0
wmm_ac_vo_aifs=2
wmm_ac_vo_cwmin=2
wmm_ac_vo_cwmax=3
wmm_ac_vo_txop_limit=47
wmm_ac_vo_acm=0

eapol_key_index_workaround=0

eap_server=0

own_ip_addr=127.0.0.1

wpa=2
wpa_passphrase={}
wpa_key_mgmt=WPA-PSK
rsn_pairwise=CCMP
", interface, ssid, password)
}

pub async fn write_config(path: &str, interface: &str, ssid: &str, password: &str) -> Result<()> {
	let config = mkconfig(interface, ssid, password);
	fs::write(path, config.as_bytes()).await?;
	Ok(())
}

// TODO: hostapd should be a child process instead of going through systemd
pub async fn restart() -> Result<()> {
	Command::new("systemctl")
		.args(&["restart", "hostapd"])
		.spawn()?
		.wait().await?;
	Ok(())
}
