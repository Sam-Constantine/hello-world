
/// 定义 web socket 请求接口
///
///
pub struct WsRequest {
    pub cmd: String,
    pub data: String,
}


impl WsRequest {
    fn new(cmd: &str, data: &str) -> Self {
        Self {
            cmd: cmd.to_string(),
            data: data.to_string(),
        }
    }

    pub fn from_str(text: &str) -> Self {
        let text = text.trim();

        let mut splitn_result = text.splitn(2, ".");
        let cmd = splitn_result.next().unwrap();
        let data = splitn_result.next();

        if let Some(data) = data {
            Self::new(cmd, data)
        } else {
            Self::new(cmd, "")
        }
    }
}

