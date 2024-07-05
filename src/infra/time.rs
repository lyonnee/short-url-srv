use std::time::{SystemTime, UNIX_EPOCH};

pub fn timestamp_millis() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH)
        .expect("时间获取失败"); 

    // 将Duration对象转换为毫秒
     since_the_epoch.as_millis() as u64
}

pub fn timestamp_secs() -> u64 {
     // 获取系统时间
     let now = SystemTime::now();

     // 将系统时间转换为UNIX时间戳（自1970年1月1日以来的秒数）
     let since_the_epoch = now.duration_since(UNIX_EPOCH)
         .expect("时间获取失败"); // 这将返回一个Duration对象
 
     // 将Duration对象转换为秒
      since_the_epoch.as_secs()
}
