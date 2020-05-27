use core::ffi::c_void;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DialogCommon {
    pub size: u32,
    /// Size of the structure
    pub language: i32,
    /// Language
    pub button_swap: i32,
    /// Set to 1 for X/O button swap
    pub graphics_thread: i32,
    /// Graphics thread priority
    pub access_thread: i32,
    /// Access/fileio thread priority (SceJobThread)
    pub font_thread: i32,
    /// Font thread priority (ScePafThread)
    pub sound_thread: i32,
    /// Sound thread priority
    pub result: i32,
    /// Result
    pub reserved: [i32; 4usize],
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum MsgDialogMode {
    Error,
    Text,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum MsgDialogPressed {
    Unknown1,
    Yes,
    No,
    Back,
}

/// Structure to hold the parameters for a message dialog
#[repr(C)]
pub struct MsgDialogParams {
    pub base: DialogCommon,
    pub unknown: i32,
    pub mode: MsgDialogMode,
    pub error_value: u32,
    /// The message to display (may contain embedded linefeeds)
    pub message: [u8; 512usize],
    pub options: i32,
    pub button_pressed: MsgDialogPressed,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NetconfAdhoc {
    pub name: [u8; 8usize],
    pub timeout: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NetconfData {
    pub base: DialogCommon,
    pub action: i32,
    /// One of NetconfActions
    pub adhocparam: *mut NetconfAdhoc,
    pub hotspot: i32,
    /// Set to 1 to allow connections with the 'Internet Browser' option set to 'Start' (ie. hotspot connection)
    pub hotspot_connected: i32,
    /// Will be set to 1 when connected to a hotspot style connection
    pub wifisp: i32,
}

/// Datatype for sce_utility_get_net_param
/// since it can return a u32 or a string
/// we use a union to avoid ugly casting
#[repr(C)]
#[derive(Copy, Clone)]
pub union NetData {
    pub as_uint: u32,
    pub as_string: [u8; 128usize],
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum SavedataMode {
    AutoLoad,
    AutoSave,
    Load,
    Save,
    ListLoad,
    ListSave,
    ListDelete,
    Delete,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum SavedataFocus {
    Unknown1,
    FirstList,
    LastList,
    Latest,
    Oldest,
    Unknown2,
    Unknown3,
    FirstEmpty,
    LastEmpty
}

/// title, savedataTitle, detail: parts of the unencrypted SFO
///data, it contains what the VSH and standard load screen shows
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SavedataSFOParam {
    pub title: [u8; 128usize],
    pub savedata_title: [u8; 128usize],
    pub detail: [u8; 1024usize],
    pub parental_level: u8,
    pub unknown: [u8; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SavedataFileData {
    pub buf: *mut c_void,
    pub buf_size: usize,
    pub size: usize,
    pub unknown: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SavedataListSaveNewData {
    pub icon0: SavedataFileData,
    pub title: *mut u8,
}

/// Structure to hold the parameters for the ::sce_utility_savedata_init_start function.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SavedataParam {
    pub base: DialogCommon,
    pub mode: SavedataMode,
    pub unknown1: i32,
    pub overwrite: i32,
    /// gameName: name used from the game for saves, equal for all saves
    pub game_name: [u8; 13usize],
    pub reserved: [u8; 3usize],
    /// saveName: name of the particular save, normally a number
    pub save_name: [u8; 20usize],
    /// saveNameList: used by multiple modes
    pub save_name_list: *mut [u8; 20usize],
    /// fileName: name of the data file of the game for example DATA.BIN
    pub file_name: [u8; 13usize],
    pub reserved1: [u8; 3usize],
    /// pointer to a buffer that will contain data file unencrypted data
    pub data_buf: *mut c_void,
    /// size of allocated space to dataBuf
    pub data_buf_size: usize,
    pub data_size: usize,
    pub sfo_param: SavedataSFOParam,
    pub icon0_file_data: SavedataFileData,
    pub icon1_file_data: SavedataFileData,
    pub pic1_file_data: SavedataFileData,
    pub snd0_file_data: SavedataFileData,
    /// Pointer to an SavedataListSaveNewData structure
    pub new_data: *mut SavedataListSaveNewData,
    /// Initial focus for lists
    pub focus: SavedataFocus,
    /// unknown2: ?
    pub unknown2: [i32; 4usize],
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum GameSharingMode {
    Single = 1,
    Multiple
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum GameSharingDataType {
    File = 1,
    Memory
}

/// Structure to hold the parameters for Game Sharing
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GameSharingParams {
    pub base: DialogCommon,
    pub unknown1: i32,
    pub unknown2: i32,
    pub name: [u8; 8usize],
    pub unknown3: i32,
    pub unknown4: i32,
    pub unknown5: i32,
    pub result: i32,
    pub filepath: *mut u8,
    pub mode: GameSharingMode,
    pub datatype: GameSharingDataType,
    pub data: *mut c_void,
    pub datasize: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HtmlViewerParam {
    pub base: DialogCommon,
    /// Pointer to the memory pool to be used
    pub memaddr: *mut c_void,
    /// Size of the memory pool
    pub memsize: u32,
    /// Unknown. Pass 0
    pub unknown1: i32,
    /// Unknown. Pass 0
    pub unknown2: i32,
    /// URL to be opened initially
    pub initialurl: *mut u8,
    /// Number of tabs (maximum of 3)
    pub numtabs: u32,
    /// One of ::HtmlViewerInterfaceModes
    pub interfacemode: u32,
    /// Values from ::HtmlViewerOptions. Bitwise OR together
    pub options: u32,
    /// Directory to be used for downloading
    pub dldirname: *mut u8,
    /// Filename to be used for downloading
    pub dlfilename: *mut u8,
    /// Directory to be used for uploading
    pub uldirname: *mut u8,
    /// Filename to be used for uploading
    pub ulfilename: *mut u8,
    /// One of ::HtmlViewerCookieModes
    pub cookiemode: u32,
    /// Unknown. Pass 0
    pub unknown3: u32,
    /// URL to set the home page to
    pub homeurl: *mut u8,
    /// One of ::HtmlViewerTextSizes
    pub textsize: u32,
    /// One of ::HtmlViewerDisplayModes
    pub displaymode: u32,
    /// One of ::HtmlViewerConnectModes
    pub connectmode: u32,
    /// One of ::HtmlViewerDisconnectModes
    pub disconnectmode: u32,
    /// The maximum amount of memory the browser used
    pub memused: u32,
    /// Unknown. Pass 0
    pub unknown4: [i32; 10usize],
}

/// OSK Field data
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OskData {
    /// Unknown. Pass 0.
    pub unk_00: i32,
    /// Unknown. Pass 0.
    pub unk_04: i32,
    /// One of ::OskInputLanguage
    pub language: i32,
    /// Unknown. Pass 0.
    pub unk_12: i32,
    /// One or more of ::OskInputType (types that are selectable by pressing SELECT)
    pub inputtype: i32,
    /// Number of lines
    pub lines: i32,
    /// Unknown. Pass 0.
    pub unk_24: i32,
    /// Description text
    pub desc: *mut u16,
    /// Initial text
    pub intext: *mut u16,
    /// Length of output text
    pub outtextlength: i32,
    /// Pointer to the output text
    pub outtext: *mut u16,
    /// Result. One of ::OskResult
    pub result: i32,
    /// The max text that can be input
    pub outtextlimit: i32,
}

/// OSK parameters
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OskParams {
    pub base: DialogCommon,
    /// Number of input fields
    pub datacount: i32,
    /// Pointer to the start of the data for the input fields
    pub data: *mut OskData,
    /// The local OSK state, one of ::OskState
    pub state: i32,
    /// Unknown. Pass 0
    pub unk_60: i32,
}


sys_lib! {
    #![name = "sceUtility"]
    #![flags = 0x4001]
    #![version = (0x00, 0x00)]

    #[psp(0x2AD8E239)]
    /// Create a message dialog
    ///
    /// # Parameters
    ///
    /// - `params`: dialog parameters
    /// # Return Value
    ///
    /// 0 on success
    pub unsafe fn sce_utility_msg_dialog_init_start(
        params: *mut MsgDialogParams,
    ) -> i32;

    #[psp(0x67AF3428)]
    /// Remove a message dialog currently active.  After calling this
    /// function you need to keep calling GetStatus and Update until
    /// you get a status of 4.
    pub unsafe fn sce_utility_msg_dialog_shutdown_start();

    #[psp(0x9A1C91D7)]
    /// Get the current status of a message dialog currently active.
    ///
    /// # Return Value
    ///
    /// 2 if the GUI is visible (you need to call sce_utility_msg_dialog_get_status).
    /// 3 if the user cancelled the dialog, and you need to call sce_utility_msg_dialog_shutdown_start.
    /// 4 if the dialog has been successfully shut down.
    pub unsafe fn sce_utility_msg_dialog_get_status() -> i32;

    #[psp(0x95FC253B)]
    /// Refresh the GUI for a message dialog currently active
    ///
    /// # Parameters
    ///
    /// - `n`: unknown, pass 1
    pub unsafe fn sce_utility_msg_dialog_update(n: i32);

    #[psp(0x4928BD96)]
    /// Abort a message dialog currently active
    pub unsafe fn sce_utility_msg_dialog_abort() -> i32;


    #[psp(0x4DB1E739)]
    /// Init the Network Configuration Dialog Utility
    ///
    /// # Parameters
    ///
    /// - `data`: pointer to NetconfData to be initialized
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_utility_netconf_init_start(data: *mut NetconfData) -> i32;

    #[psp(0xF88155F6)]
    /// Shutdown the Network Configuration Dialog Utility
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_utility_netconf_shutdown_start() -> i32;

    #[psp(0x91E70E35)]
    /// Update the Network Configuration Dialog GUI
    ///
    /// # Parameters
    ///
    /// - `unknown`: unknown; set to 1
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_utility_netconf_update(unknown: i32) -> i32;

    #[psp(0x6332AA39)]
    /// Get the status of a running Network Configuration Dialog
    ///
    /// # Return Value
    ///
    /// one of DialogState on success, < 0 on error
    pub unsafe fn sce_utility_netconf_get_status() -> i32;

    #[psp(0x5EEE6548)]
    /// Check existance of a Net Configuration
    ///
    /// # Parameters
    ///
    /// - `id`: id of net Configuration (1 to n)
    /// # Return Value
    ///
    /// 0 on success,
    pub unsafe fn sce_utility_check_net_param(id: i32) -> i32;

    #[psp(0x434D4B3A)]
    /// Get Net Configuration Parameter
    ///
    /// # Parameters
    ///
    /// - `conf`: Net Configuration number (1 to n)
    /// (0 returns valid but seems to be a copy of the last config requested)
    /// # Parameters
    ///
    /// - `param`: which parameter to get
    /// - `data`: parameter data
    /// # Return Value
    ///
    /// 0 on success,
    pub unsafe fn sce_utility_get_net_param(
        conf: i32,
        param: i32,
        data: *mut NetData,
    ) -> i32;

    #[psp(0x50C4CD57)]
    /// Saves or Load savedata to/from the passed structure
    /// After having called this continue calling sce_utility_savedata_get_status to
    /// check if the operation is completed
    ///
    /// # Parameters
    ///
    /// - `params`: savedata parameters
    /// # Return Value
    ///
    /// 0 on success
    pub unsafe fn sce_utility_savedata_init_start(
        params: *mut SavedataParam,
    ) -> i32;

    #[psp(0x8874DBE0)]
    /// Check the current status of the saving/loading/shutdown process
    /// Continue calling this to check current status of the process
    /// before calling this call also sce_utility_savedata_update
    /// # Return Value
    ///
    /// - 2 if the process is still being processed.
    /// - 3 on save/load success, then you can call sce_utility_savedata_shutdown_start.
    /// - 4 on complete shutdown.
    pub unsafe fn sce_utility_savedata_get_status() -> i32;

    #[psp(0x9790B33C)]
    /// Shutdown the savedata utility. after calling this continue calling
    /// sce_utility_savedata_get_status to check when it has shutdown
    ///
    /// # Return Value
    ///
    /// 0 on success
    ///
    pub unsafe fn sce_utility_savedata_shutdown_start() -> i32;

    #[psp(0xD4B95FFB)]
    /// Refresh status of the savedata function
    ///
    /// # Parameters
    ///
    /// - `unknown`: unknown, pass 1
    pub unsafe fn sce_utility_savedata_update(unknown: i32);

    #[psp(0xC492F751)]
    /// Init the game sharing
    ///
    /// # Parameters
    ///
    /// - `params`: game sharing parameters
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub unsafe fn sce_utility_game_sharing_init_start(
        params: *mut GameSharingParams,
    ) -> i32;

    #[psp(0xEFC6F80F)]
    /// Shutdown game sharing.
    pub unsafe fn sce_utility_game_sharing_shutdown_start();

    #[psp(0x946963F3)]
    /// Get the current status of game sharing.
    ///
    /// # Return Value
    ///
    /// 2 if the GUI is visible (you need to call sce_utility_game_sharing_get_status).
    /// 3 if the user cancelled the dialog, and you need to call
    ///   sce_utility_game_sharing_shutdown_start.
    /// 4 if the dialog has been successfully shut down.
    pub unsafe fn sce_utility_game_sharing_get_status() -> i32;

    #[psp(0x7853182D)]
    /// Refresh the GUI for game sharing
    ///
    /// # Parameters
    ///
    /// - `n`: unknown, pass 1
    pub unsafe fn sce_utility_game_sharing_update(n: i32);

    #[psp(0xCDC3AA41)]
    /// Init the html viewer
    ///
    /// # Parameters
    ///
    /// - `params`: html viewer parameters
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error.
    pub unsafe fn sce_utility_html_viewer_init_start(
        params: *mut HtmlViewerParam,
    ) -> i32;

    #[psp(0xF5CE1134)]
    /// Shutdown html viewer.
    pub unsafe fn sce_utility_html_viewer_shutdown_start() -> i32;

    #[psp(0x05AFB9E4)]
    /// Refresh the GUI for html viewer
    ///
    /// # Parameters
    ///
    /// - `n`: unknown, pass 1
    pub unsafe fn sce_utility_html_viewer_update(n: i32) -> i32;

    #[psp(0xBDA7D894)]
    /// Get the current status of the html viewer.
    ///
    /// # Return Value
    ///
    /// 2 if the GUI is visible (you need to call sce_utility_html_viewer_get_status).
    /// 3 if the user cancelled the dialog, and you need to call
    ///   sce_utility_html_viewer_shutdown_start.
    /// 4 if the dialog has been successfully shut down.
    pub unsafe fn sce_utility_html_viewer_get_status() -> i32;

    #[psp(0x45C18506)]
    /// Set Integer System Parameter
    ///
    /// # Parameters
    ///
    /// - `id`: which parameter to set
    /// - `value`: integer value to set
    /// # Return Value
    ///
    /// 0 on success, PSP_SYSTEMPARAM_RETVAL_FAIL on failure
    pub unsafe fn sce_utility_set_system_param_int(
        id: i32,
        value: i32,
    ) -> i32;

    #[psp(0x41E30674)]
    /// Set String System Parameter
    ///
    /// # Parameters
    ///
    /// - `id`: which parameter to set
    /// - `str`: char * value to set
    /// # Return Value
    ///
    /// 0 on success, PSP_SYSTEMPARAM_RETVAL_FAIL on failure
    pub unsafe fn sce_utility_set_system_param_string(
        id: i32,
        str: *const u8,
    ) -> i32;

    #[psp(0xA5DA2406)]
    /// Get Integer System Parameter
    ///
    /// # Parameters
    ///
    /// - `id`: which parameter to get
    /// - `value`: pointer to integer value to place result in
    /// # Return Value
    ///
    /// 0 on success, PSP_SYSTEMPARAM_RETVAL_FAIL on failure
    pub unsafe fn sce_utility_get_system_param_int(
        id: i32,
        value: *mut i32,
    ) -> i32;

    #[psp(0x34B78343)]
    /// Get String System Parameter
    ///
    /// # Parameters
    ///
    /// - `id`: which parameter to get
    /// - `str`: char * buffer to place result in
    /// - `len`: length of str buffer
    /// # Return Value
    ///
    /// 0 on success, PSP_SYSTEMPARAM_RETVAL_FAIL on failure
    pub unsafe fn sce_utility_get_system_param_string(
        id: i32,
        str: *mut u8,
        len: i32,
    ) -> i32;

    #[psp(0xF6269B82)]
    /// Create an on-screen keyboard
    ///
    /// # Parameters
    ///
    /// - `params`: OSK parameters.
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub unsafe fn sce_utility_osk_init_start(params: *mut OskParams) -> i32;

    #[psp(0x3DFAEBA9)]
    /// Remove a currently active keyboard. After calling this function you must
    ///
    /// poll sce_utility_osk_get_status() until it returns None
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub unsafe fn sce_utility_osk_shutdown_start() -> i32;

    #[psp(0x4B85C861)]
    /// Refresh the GUI for a keyboard currently active
    ///
    /// # Parameters
    ///
    /// - `n`: Unknown, pass 1.
    ///
    /// # Return Value
    ///
    /// < 0 on error.
    pub unsafe fn sce_utility_osk_update(n: i32) -> i32;

    #[psp(0xF3F76017)]
    /// Get the status of a on-screen keyboard currently active.
    ///
    /// # Return Value
    ///
    /// the current status of the keyboard. See ::DialogState for details.
    pub unsafe fn sce_utility_osk_get_status() -> i32;

    #[psp(0x1579a159)]
    /// Load a network module (PRX) from user mode.
    /// Load PSP_NET_MODULE_COMMON and PSP_NET_MODULE_INET
    /// to use infrastructure WifI (via an access point).
    /// Available on firmware 2.00 and higher only.
    ///
    /// # Parameters
    ///
    /// - `module`: module number to load (PSP_NET_MODULE_xxx)
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_utility_load_net_module(module: i32) -> i32;

    #[psp(0x64d50c56)]
    /// Unload a network module (PRX) from user mode.
    /// Available on firmware 2.00 and higher only.
    ///
    /// # Parameters
    ///
    /// - `module`: module number be unloaded
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_utility_unload_net_module(module: i32) -> i32;

    #[psp(0xC629AF26)]
    /// Load an audio/video module (PRX) from user mode.
    ///
    /// Available on firmware 2.00 and higher only.
    ///
    /// # Parameters
    ///
    /// - `module`: module number to load (PSP_AV_MODULE_xxx)
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_utility_load_av_module(module: i32) -> i32;

    #[psp(0xF7D8D092)]
    /// Unload an audio/video module (PRX) from user mode.
    /// Available on firmware 2.00 and higher only.
    ///
    /// # Parameters
    ///
    /// - `module`: module number to be unloaded
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_utility_unload_av_module(module: i32) -> i32;

    #[psp(0x0D5BC6D2)]
    /// Load a usb module (PRX) from user mode.
    /// Available on firmware 2.70 and higher only.
    ///
    /// # Parameters
    ///
    /// - `module`: module number to load (PSP_USB_MODULE_xxx)
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_utility_load_usb_module(module: i32) -> i32;

    #[psp(0xF64910F0)]
    /// Unload a usb module (PRX) from user mode.
    /// Available on firmware 2.70 and higher only.
    ///
    /// # Parameters
    ///
    /// - `module`: module number to be unloaded
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_utility_unload_usb_module(module: i32) -> i32;

    #[psp(0x2A2B3DE0)]
    /// Load a module (PRX) from user mode.
    ///
    /// # Parameters
    ///
    /// - `module`: module to load (PSP_MODULE_xxx)
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_utility_load_module(module: i32) -> i32;

    #[psp(0xE49BFE92)]
    /// Unload a module (PRX) from user mode.
    ///
    /// # Parameters
    ///
    /// - `module`: module to unload (PSP_MODULE_xxx)
    ///
    /// # Return Value
    ///
    /// 0 on success, < 0 on error
    pub unsafe fn sce_utility_unload_module(module: i32) -> i32;

}

sys_lib! {
    #![name = "sceUtility_netparam_internal"]
    #![flags = 0x4001]
    #![version = (0x00, 0x00)]


    #[psp(0x072DEBF2)]
    /// Create a new Network Configuration
    /// @note This creates a new configuration at conf and clears 0
    ///
    /// # Parameters
    ///
    /// - `conf`: Net Configuration number (1 to n)
    ///
    /// # Return Value
    ///
    /// 0 on success
    pub unsafe fn sce_utility_create_net_param(conf: i32) -> i32;

    #[psp(0xFC4516F3)]
    /// Sets a network parameter
    /// 
    /// # Note
    /// 
    /// This sets only to configuration 0
    ///
    /// # Parameters
    ///
    /// - `param`: Which parameter to set
    /// - `val`: Pointer to the the data to set
    ///
    /// # Return Value
    ///
    /// 0 on success
    pub unsafe fn sce_utility_set_net_param(
        param: i32,
        val: *const c_void,
    ) -> i32;

    #[psp(0xFB0C4840)]
    /// Copies a Network Configuration to another
    ///
    /// # Parameters
    ///
    /// - `src`: Source Net Configuration number (0 to n)
    /// - `dest`: Destination Net Configuration number (0 to n)
    ///
    /// # Return Value
    ///
    /// 0 on success
    pub unsafe fn sce_utility_copy_net_param(
        src: i32,
        dest: i32,
    ) -> i32;

    #[psp(0x9CE50172)]
    /// Deletes a Network Configuration
    ///
    /// # Parameters
    ///
    /// - `conf`: Net Configuration number (1 to n)
    ///
    /// # Return Value
    ///
    /// 0 on success
    pub unsafe fn sce_utility_delete_net_param(conf: i32) -> i32;

}
