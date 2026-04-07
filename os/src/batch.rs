use core::arch::asm;

use lazy_static::lazy_static;

use crate::sync::up::RcUpSafeCell;

const MAX_APP_NUM: usize = 16;
const APP_START_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;

/// APP 管理器
///
/// 用于管理app加载，运行和切换。
///
/// ***这里和教程中不一致，这里是按照实际的变量意义重新修改了变量名称，还有这里直接使用了MAX_APP_NUM，因为这里就5个APP，16个完全可以覆盖。***
///
/// total_apps: app的总数
/// current_app: 当前运行的APP，
/// app_address: 存储app起始和结束的地址
struct AppManager {
    total_apps: usize,
    current_app: usize,
    app_address: [usize; MAX_APP_NUM],
}

impl AppManager {
    // 打印所有的app
    pub fn print_app_info(&self) {
        for i in 0..self.total_apps {
            println!(
                "[kernel] app_{} at: [{:#x},  {:#x})",
                i,
                self.app_address[i],
                self.app_address[i + 1]
            );
        }
    }

    // 获取当前app的id
    pub fn get_current_app(&self) -> usize {
        self.current_app
    }

    // 更新到下一个app的id
    pub fn move_to_next_app(&mut self) {
        self.current_app += 1;
    }

    // 根据APPID加载对应的app
    fn load_app(&self, app_id: usize) {
        if self.total_apps < app_id {
            panic!("[kernel] All applications compeleted!")
        }

        println!("[kernel] Loading app_{}", app_id);
        let app_size = self.app_address[app_id + 1] - self.app_address[app_id];
        if app_size > APP_SIZE_LIMIT {
            panic!("[kernel] App size to large")
        }

        unsafe {
            core::slice::from_raw_parts_mut(APP_START_ADDRESS as *mut u8, APP_SIZE_LIMIT).fill(0);
            let app_src =
                core::slice::from_raw_parts(self.app_address[app_id] as *const u8, app_size);
            let app_dst =
                core::slice::from_raw_parts_mut(APP_START_ADDRESS as *mut u8, app_src.len());
            app_dst.copy_from_slice(app_src);

            // 强制刷新cpu缓存
            asm!("fence.i");
        }
    }
}

// 这里也是和教程有出入。
lazy_static! {
    static ref APP_MANAGER: RcUpSafeCell<AppManager> = {
        unsafe {
            unsafe extern "C" { static _num_app: usize; }
            let num_app_ptr = _num_app as *const usize;
            let total_apps = num_app_ptr.read_volatile();
            let mut app_address = [0; MAX_APP_NUM];

            // 这里是获取汇编中_num_app中所有的开始和结束地址，_num_app的内存是一个连续的内存地址，
            // rust中使用from_raw_parts是按照8字节的宽度解析，因此这里解析了所有的地址。
            let address = core::slice::from_raw_parts(num_app_ptr.add(1), total_apps + 1);
            // 这里是实际复制。只是复制了起始地址。
            app_address[0..=total_apps].copy_from_slice(address);
            RcUpSafeCell::new(
                AppManager { total_apps, current_app: 0, app_address }
            )
        }
    };
}

pub fn init() {
    print_app_info()
}

pub fn print_app_info() {
    APP_MANAGER.execute_access().print_app_info();
}
