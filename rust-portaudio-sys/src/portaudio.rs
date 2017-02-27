#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub const paFormatIsSupported: ::std::os::raw::c_uint = 0;
pub const paFramesPerBufferUnspecified: ::std::os::raw::c_uint = 0;
extern "C" {
    /** Retrieve the release number of the currently running PortAudio build.
 For example, for version "19.5.1" this will return 0x00130501.

 @see paMakeVersionNumber
*/
    pub fn Pa_GetVersion() -> ::std::os::raw::c_int;
}
extern "C" {
    /** Retrieve a textual description of the current PortAudio build,
 e.g. "PortAudio V19.5.0-devel, revision 1952M".
 The format of the text may change in the future. Do not try to parse the
 returned string.

 @deprecated As of 19.5.0, use Pa_GetVersionInfo()->versionText instead.
*/
    pub fn Pa_GetVersionText() -> *const ::std::os::raw::c_char;
}
/**
 A structure containing PortAudio API version information.
 @see Pa_GetVersionInfo, paMakeVersionNumber
 @version Available as of 19.5.0.
*/
#[repr(C)]
#[derive(Debug, Copy)]
pub struct PaVersionInfo {
    pub versionMajor: ::std::os::raw::c_int,
    pub versionMinor: ::std::os::raw::c_int,
    pub versionSubMinor: ::std::os::raw::c_int,
    /**
     This is currently the Git revision hash but may change in the future.
     The versionControlRevision is updated by running a script before compiling the library.
     If the update does not occur, this value may refer to an earlier revision.
    */
    pub versionControlRevision: *const ::std::os::raw::c_char,
    /** Version as a string, for example "PortAudio V19.5.0-devel, revision 1952M" */
    pub versionText: *const ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_PaVersionInfo() {
    assert_eq!(::std::mem::size_of::<PaVersionInfo>() , 32usize , concat ! (
               "Size of: " , stringify ! ( PaVersionInfo ) ));
    assert_eq! (::std::mem::align_of::<PaVersionInfo>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( PaVersionInfo ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaVersionInfo ) ) . versionMajor as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( PaVersionInfo ) , "::"
                , stringify ! ( versionMajor ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaVersionInfo ) ) . versionMinor as *
                const _ as usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( PaVersionInfo ) , "::"
                , stringify ! ( versionMinor ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaVersionInfo ) ) . versionSubMinor as *
                const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( PaVersionInfo ) , "::"
                , stringify ! ( versionSubMinor ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaVersionInfo ) ) .
                versionControlRevision as * const _ as usize } , 16usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( PaVersionInfo ) , "::"
                , stringify ! ( versionControlRevision ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaVersionInfo ) ) . versionText as *
                const _ as usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( PaVersionInfo ) , "::"
                , stringify ! ( versionText ) ));
}
impl Clone for PaVersionInfo {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    /** Retrieve version information for the currently running PortAudio build.
 @return A pointer to an immutable PaVersionInfo structure.

 @note This function can be called at any time. It does not require PortAudio
 to be initialized. The structure pointed to is statically allocated. Do not
 attempt to free it or modify it.

 @see PaVersionInfo, paMakeVersionNumber
 @version Available as of 19.5.0.
*/
    pub fn Pa_GetVersionInfo() -> *const PaVersionInfo;
}
/** Error codes returned by PortAudio functions.
 Note that with the exception of paNoError, all PaErrorCodes are negative.
*/
pub type PaError = ::std::os::raw::c_int;
enum_from_primitive! {
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PaErrorCode {
    paNoError = 0,
    paNotInitialized = -10000,
    paUnanticipatedHostError = -9999,
    paInvalidChannelCount = -9998,
    paInvalidSampleRate = -9997,
    paInvalidDevice = -9996,
    paInvalidFlag = -9995,
    paSampleFormatNotSupported = -9994,
    paBadIODeviceCombination = -9993,
    paInsufficientMemory = -9992,
    paBufferTooBig = -9991,
    paBufferTooSmall = -9990,
    paNullCallback = -9989,
    paBadStreamPtr = -9988,
    paTimedOut = -9987,
    paInternalError = -9986,
    paDeviceUnavailable = -9985,
    paIncompatibleHostApiSpecificStreamInfo = -9984,
    paStreamIsStopped = -9983,
    paStreamIsNotStopped = -9982,
    paInputOverflowed = -9981,
    paOutputUnderflowed = -9980,
    paHostApiNotFound = -9979,
    paInvalidHostApi = -9978,
    paCanNotReadFromACallbackStream = -9977,
    paCanNotWriteToACallbackStream = -9976,
    paCanNotReadFromAnOutputOnlyStream = -9975,
    paCanNotWriteToAnInputOnlyStream = -9974,
    paIncompatibleStreamHostApi = -9973,
    paBadBufferPtr = -9972,
}
}
extern "C" {
    /** Translate the supplied PortAudio error code into a human readable
 message.
*/
    pub fn Pa_GetErrorText(errorCode: PaError)
     -> *const ::std::os::raw::c_char;
}
extern "C" {
    /** Library initialization function - call this before using PortAudio.
 This function initializes internal data structures and prepares underlying
 host APIs for use.  With the exception of Pa_GetVersion(), Pa_GetVersionText(),
 and Pa_GetErrorText(), this function MUST be called before using any other
 PortAudio API functions.

 If Pa_Initialize() is called multiple times, each successful 
 call must be matched with a corresponding call to Pa_Terminate(). 
 Pairs of calls to Pa_Initialize()/Pa_Terminate() may overlap, and are not 
 required to be fully nested.

 Note that if Pa_Initialize() returns an error code, Pa_Terminate() should
 NOT be called.

 @return paNoError if successful, otherwise an error code indicating the cause
 of failure.

 @see Pa_Terminate
*/
    pub fn Pa_Initialize() -> PaError;
}
extern "C" {
    /** Library termination function - call this when finished using PortAudio.
 This function deallocates all resources allocated by PortAudio since it was
 initialized by a call to Pa_Initialize(). In cases where Pa_Initialise() has
 been called multiple times, each call must be matched with a corresponding call
 to Pa_Terminate(). The final matching call to Pa_Terminate() will automatically
 close any PortAudio streams that are still open.

 Pa_Terminate() MUST be called before exiting a program which uses PortAudio.
 Failure to do so may result in serious resource leaks, such as audio devices
 not being available until the next reboot.

 @return paNoError if successful, otherwise an error code indicating the cause
 of failure.
 
 @see Pa_Initialize
*/
    pub fn Pa_Terminate() -> PaError;
}
/** The type used to refer to audio devices. Values of this type usually
 range from 0 to (Pa_GetDeviceCount()-1), and may also take on the PaNoDevice
 and paUseHostApiSpecificDeviceSpecification values.

 @see Pa_GetDeviceCount, paNoDevice, paUseHostApiSpecificDeviceSpecification
*/
pub type PaDeviceIndex = ::std::os::raw::c_int;
/** The type used to enumerate to host APIs at runtime. Values of this type
 range from 0 to (Pa_GetHostApiCount()-1).

 @see Pa_GetHostApiCount
*/
pub type PaHostApiIndex = ::std::os::raw::c_int;
extern "C" {
    /** Retrieve the number of available host APIs. Even if a host API is
 available it may have no devices available.

 @return A non-negative value indicating the number of available host APIs
 or, a PaErrorCode (which are always negative) if PortAudio is not initialized
 or an error is encountered.

 @see PaHostApiIndex
*/
    pub fn Pa_GetHostApiCount() -> PaHostApiIndex;
}
extern "C" {
    /** Retrieve the index of the default host API. The default host API will be
 the lowest common denominator host API on the current platform and is
 unlikely to provide the best performance.

 @return A non-negative value ranging from 0 to (Pa_GetHostApiCount()-1)
 indicating the default host API index or, a PaErrorCode (which are always
 negative) if PortAudio is not initialized or an error is encountered.
*/
    pub fn Pa_GetDefaultHostApi() -> PaHostApiIndex;
}
enum_from_primitive! {
#[repr(u32)]
/** Unchanging unique identifiers for each supported host API. This type
 is used in the PaHostApiInfo structure. The values are guaranteed to be
 unique and to never change, thus allowing code to be written that
 conditionally uses host API specific extensions.

 New type ids will be allocated when support for a host API reaches
 "public alpha" status, prior to that developers should use the
 paInDevelopment type id.

 @see PaHostApiInfo
*/
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PaHostApiTypeId {
    paInDevelopment = 0,
    paDirectSound = 1,
    paMME = 2,
    paASIO = 3,
    paSoundManager = 4,
    paCoreAudio = 5,
    paOSS = 7,
    paALSA = 8,
    paAL = 9,
    paBeOS = 10,
    paWDMKS = 11,
    paJACK = 12,
    paWASAPI = 13,
    paAudioScienceHPI = 14,
}
}
/** A structure containing information about a particular host API. */
#[repr(C)]
#[derive(Debug, Copy)]
pub struct PaHostApiInfo {
    /** this is struct version 1 */
    pub structVersion: ::std::os::raw::c_int,
    /** The well known unique identifier of this host API @see PaHostApiTypeId */
    pub type_: PaHostApiTypeId,
    /** A textual description of the host API for display on user interfaces. */
    pub name: *const ::std::os::raw::c_char,
    /**  The number of devices belonging to this host API. This field may be
     used in conjunction with Pa_HostApiDeviceIndexToDeviceIndex() to enumerate
     all devices for this host API.
     @see Pa_HostApiDeviceIndexToDeviceIndex
    */
    pub deviceCount: ::std::os::raw::c_int,
    /** The default input device for this host API. The value will be a
     device index ranging from 0 to (Pa_GetDeviceCount()-1), or paNoDevice
     if no default input device is available.
    */
    pub defaultInputDevice: PaDeviceIndex,
    /** The default output device for this host API. The value will be a
     device index ranging from 0 to (Pa_GetDeviceCount()-1), or paNoDevice
     if no default output device is available.
    */
    pub defaultOutputDevice: PaDeviceIndex,
}
#[test]
fn bindgen_test_layout_PaHostApiInfo() {
    assert_eq!(::std::mem::size_of::<PaHostApiInfo>() , 32usize , concat ! (
               "Size of: " , stringify ! ( PaHostApiInfo ) ));
    assert_eq! (::std::mem::align_of::<PaHostApiInfo>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( PaHostApiInfo ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaHostApiInfo ) ) . structVersion as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( PaHostApiInfo ) , "::"
                , stringify ! ( structVersion ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaHostApiInfo ) ) . type_ as * const _ as
                usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( PaHostApiInfo ) , "::"
                , stringify ! ( type_ ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaHostApiInfo ) ) . name as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( PaHostApiInfo ) , "::"
                , stringify ! ( name ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaHostApiInfo ) ) . deviceCount as *
                const _ as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( PaHostApiInfo ) , "::"
                , stringify ! ( deviceCount ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaHostApiInfo ) ) . defaultInputDevice as
                * const _ as usize } , 20usize , concat ! (
                "Alignment of field: " , stringify ! ( PaHostApiInfo ) , "::"
                , stringify ! ( defaultInputDevice ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaHostApiInfo ) ) . defaultOutputDevice
                as * const _ as usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( PaHostApiInfo ) , "::"
                , stringify ! ( defaultOutputDevice ) ));
}
impl Clone for PaHostApiInfo {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    /** Retrieve a pointer to a structure containing information about a specific
 host Api.

 @param hostApi A valid host API index ranging from 0 to (Pa_GetHostApiCount()-1)

 @return A pointer to an immutable PaHostApiInfo structure describing
 a specific host API. If the hostApi parameter is out of range or an error
 is encountered, the function returns NULL.

 The returned structure is owned by the PortAudio implementation and must not
 be manipulated or freed. The pointer is only guaranteed to be valid between
 calls to Pa_Initialize() and Pa_Terminate().
*/
    pub fn Pa_GetHostApiInfo(hostApi: PaHostApiIndex) -> *const PaHostApiInfo;
}
extern "C" {
    /** Convert a static host API unique identifier, into a runtime
 host API index.

 @param type A unique host API identifier belonging to the PaHostApiTypeId
 enumeration.

 @return A valid PaHostApiIndex ranging from 0 to (Pa_GetHostApiCount()-1) or,
 a PaErrorCode (which are always negative) if PortAudio is not initialized
 or an error is encountered.
 
 The paHostApiNotFound error code indicates that the host API specified by the
 type parameter is not available.

 @see PaHostApiTypeId
*/
    pub fn Pa_HostApiTypeIdToHostApiIndex(type_: PaHostApiTypeId)
     -> PaHostApiIndex;
}
extern "C" {
    /** Convert a host-API-specific device index to standard PortAudio device index.
 This function may be used in conjunction with the deviceCount field of
 PaHostApiInfo to enumerate all devices for the specified host API.

 @param hostApi A valid host API index ranging from 0 to (Pa_GetHostApiCount()-1)

 @param hostApiDeviceIndex A valid per-host device index in the range
 0 to (Pa_GetHostApiInfo(hostApi)->deviceCount-1)

 @return A non-negative PaDeviceIndex ranging from 0 to (Pa_GetDeviceCount()-1)
 or, a PaErrorCode (which are always negative) if PortAudio is not initialized
 or an error is encountered.

 A paInvalidHostApi error code indicates that the host API index specified by
 the hostApi parameter is out of range.

 A paInvalidDevice error code indicates that the hostApiDeviceIndex parameter
 is out of range.
 
 @see PaHostApiInfo
*/
    pub fn Pa_HostApiDeviceIndexToDeviceIndex(hostApi: PaHostApiIndex,
                                              hostApiDeviceIndex:
                                                  ::std::os::raw::c_int)
     -> PaDeviceIndex;
}
/** Structure used to return information about a host error condition.
*/
#[repr(C)]
#[derive(Debug, Copy)]
pub struct PaHostErrorInfo {
    /**< the host API which returned the error code */
    pub hostApiType: PaHostApiTypeId,
    /**< the error code returned */
    pub errorCode: ::std::os::raw::c_long,
    /**< a textual description of the error if available, otherwise a zero-length string */
    pub errorText: *const ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_PaHostErrorInfo() {
    assert_eq!(::std::mem::size_of::<PaHostErrorInfo>() , 24usize , concat ! (
               "Size of: " , stringify ! ( PaHostErrorInfo ) ));
    assert_eq! (::std::mem::align_of::<PaHostErrorInfo>() , 8usize , concat !
                ( "Alignment of " , stringify ! ( PaHostErrorInfo ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaHostErrorInfo ) ) . hostApiType as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( PaHostErrorInfo ) ,
                "::" , stringify ! ( hostApiType ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaHostErrorInfo ) ) . errorCode as *
                const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( PaHostErrorInfo ) ,
                "::" , stringify ! ( errorCode ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaHostErrorInfo ) ) . errorText as *
                const _ as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( PaHostErrorInfo ) ,
                "::" , stringify ! ( errorText ) ));
}
impl Clone for PaHostErrorInfo {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    /** Return information about the last host error encountered. The error
 information returned by Pa_GetLastHostErrorInfo() will never be modified
 asynchronously by errors occurring in other PortAudio owned threads
 (such as the thread that manages the stream callback.)

 This function is provided as a last resort, primarily to enhance debugging
 by providing clients with access to all available error information.

 @return A pointer to an immutable structure constraining information about
 the host error. The values in this structure will only be valid if a
 PortAudio function has previously returned the paUnanticipatedHostError
 error code.
*/
    pub fn Pa_GetLastHostErrorInfo() -> *const PaHostErrorInfo;
}
extern "C" {
    /** Retrieve the number of available devices. The number of available devices
 may be zero.

 @return A non-negative value indicating the number of available devices or,
 a PaErrorCode (which are always negative) if PortAudio is not initialized
 or an error is encountered.
*/
    pub fn Pa_GetDeviceCount() -> PaDeviceIndex;
}
extern "C" {
    /** Retrieve the index of the default input device. The result can be
 used in the inputDevice parameter to Pa_OpenStream().

 @return The default input device index for the default host API, or paNoDevice
 if no default input device is available or an error was encountered.
*/
    pub fn Pa_GetDefaultInputDevice() -> PaDeviceIndex;
}
extern "C" {
    /** Retrieve the index of the default output device. The result can be
 used in the outputDevice parameter to Pa_OpenStream().

 @return The default output device index for the default host API, or paNoDevice
 if no default output device is available or an error was encountered.

 @note
 On the PC, the user can specify a default device by
 setting an environment variable. For example, to use device #1.
<pre>
 set PA_RECOMMENDED_OUTPUT_DEVICE=1
</pre>
 The user should first determine the available device ids by using
 the supplied application "pa_devs".
*/
    pub fn Pa_GetDefaultOutputDevice() -> PaDeviceIndex;
}
/** The type used to represent monotonic time in seconds. PaTime is 
 used for the fields of the PaStreamCallbackTimeInfo argument to the 
 PaStreamCallback and as the result of Pa_GetStreamTime().

 PaTime values have unspecified origin.
     
 @see PaStreamCallback, PaStreamCallbackTimeInfo, Pa_GetStreamTime
*/
pub type PaTime = f64;
/** A type used to specify one or more sample formats. Each value indicates
 a possible format for sound data passed to and from the stream callback,
 Pa_ReadStream and Pa_WriteStream.

 The standard formats paFloat32, paInt16, paInt32, paInt24, paInt8
 and aUInt8 are usually implemented by all implementations.

 The floating point representation (paFloat32) uses +1.0 and -1.0 as the
 maximum and minimum respectively.

 paUInt8 is an unsigned 8 bit format where 128 is considered "ground"

 The paNonInterleaved flag indicates that audio data is passed as an array 
 of pointers to separate buffers, one buffer for each channel. Usually,
 when this flag is not used, audio data is passed as a single buffer with
 all channels interleaved.

 @see Pa_OpenStream, Pa_OpenDefaultStream, PaDeviceInfo
 @see paFloat32, paInt16, paInt32, paInt24, paInt8
 @see paUInt8, paCustomFormat, paNonInterleaved
*/
pub type PaSampleFormat = ::std::os::raw::c_ulong;
/** A structure providing information and capabilities of PortAudio devices.
 Devices may support input, output or both input and output.
*/
#[repr(C)]
#[derive(Debug, Copy)]
pub struct PaDeviceInfo {
    pub structVersion: ::std::os::raw::c_int,
    pub name: *const ::std::os::raw::c_char,
    /**< note this is a host API index, not a type id*/
    pub hostApi: PaHostApiIndex,
    pub maxInputChannels: ::std::os::raw::c_int,
    pub maxOutputChannels: ::std::os::raw::c_int,
    /** Default latency values for interactive performance. */
    pub defaultLowInputLatency: PaTime,
    pub defaultLowOutputLatency: PaTime,
    /** Default latency values for robust non-interactive applications (eg. playing sound files). */
    pub defaultHighInputLatency: PaTime,
    pub defaultHighOutputLatency: PaTime,
    pub defaultSampleRate: f64,
}
#[test]
fn bindgen_test_layout_PaDeviceInfo() {
    assert_eq!(::std::mem::size_of::<PaDeviceInfo>() , 72usize , concat ! (
               "Size of: " , stringify ! ( PaDeviceInfo ) ));
    assert_eq! (::std::mem::align_of::<PaDeviceInfo>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( PaDeviceInfo ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaDeviceInfo ) ) . structVersion as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( PaDeviceInfo ) , "::" ,
                stringify ! ( structVersion ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaDeviceInfo ) ) . name as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( PaDeviceInfo ) , "::" ,
                stringify ! ( name ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaDeviceInfo ) ) . hostApi as * const _
                as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( PaDeviceInfo ) , "::" ,
                stringify ! ( hostApi ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaDeviceInfo ) ) . maxInputChannels as *
                const _ as usize } , 20usize , concat ! (
                "Alignment of field: " , stringify ! ( PaDeviceInfo ) , "::" ,
                stringify ! ( maxInputChannels ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaDeviceInfo ) ) . maxOutputChannels as *
                const _ as usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( PaDeviceInfo ) , "::" ,
                stringify ! ( maxOutputChannels ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaDeviceInfo ) ) . defaultLowInputLatency
                as * const _ as usize } , 32usize , concat ! (
                "Alignment of field: " , stringify ! ( PaDeviceInfo ) , "::" ,
                stringify ! ( defaultLowInputLatency ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaDeviceInfo ) ) .
                defaultLowOutputLatency as * const _ as usize } , 40usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( PaDeviceInfo ) , "::" ,
                stringify ! ( defaultLowOutputLatency ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaDeviceInfo ) ) .
                defaultHighInputLatency as * const _ as usize } , 48usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( PaDeviceInfo ) , "::" ,
                stringify ! ( defaultHighInputLatency ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaDeviceInfo ) ) .
                defaultHighOutputLatency as * const _ as usize } , 56usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( PaDeviceInfo ) , "::" ,
                stringify ! ( defaultHighOutputLatency ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaDeviceInfo ) ) . defaultSampleRate as *
                const _ as usize } , 64usize , concat ! (
                "Alignment of field: " , stringify ! ( PaDeviceInfo ) , "::" ,
                stringify ! ( defaultSampleRate ) ));
}
impl Clone for PaDeviceInfo {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    /** Retrieve a pointer to a PaDeviceInfo structure containing information
 about the specified device.
 @return A pointer to an immutable PaDeviceInfo structure. If the device
 parameter is out of range the function returns NULL.

 @param device A valid device index in the range 0 to (Pa_GetDeviceCount()-1)

 @note PortAudio manages the memory referenced by the returned pointer,
 the client must not manipulate or free the memory. The pointer is only
 guaranteed to be valid between calls to Pa_Initialize() and Pa_Terminate().

 @see PaDeviceInfo, PaDeviceIndex
*/
    pub fn Pa_GetDeviceInfo(device: PaDeviceIndex) -> *const PaDeviceInfo;
}
/** Parameters for one direction (input or output) of a stream.
*/
#[repr(C)]
#[derive(Debug, Copy)]
pub struct PaStreamParameters {
    /** A valid device index in the range 0 to (Pa_GetDeviceCount()-1)
     specifying the device to be used or the special constant
     paUseHostApiSpecificDeviceSpecification which indicates that the actual
     device(s) to use are specified in hostApiSpecificStreamInfo.
     This field must not be set to paNoDevice.
    */
    pub device: PaDeviceIndex,
    /** The number of channels of sound to be delivered to the
     stream callback or accessed by Pa_ReadStream() or Pa_WriteStream().
     It can range from 1 to the value of maxInputChannels in the
     PaDeviceInfo record for the device specified by the device parameter.
    */
    pub channelCount: ::std::os::raw::c_int,
    /** The sample format of the buffer provided to the stream callback,
     a_ReadStream() or Pa_WriteStream(). It may be any of the formats described
     by the PaSampleFormat enumeration.
    */
    pub sampleFormat: PaSampleFormat,
    /** The desired latency in seconds. Where practical, implementations should
     configure their latency based on these parameters, otherwise they may
     choose the closest viable latency instead. Unless the suggested latency
     is greater than the absolute upper limit for the device implementations
     should round the suggestedLatency up to the next practical value - ie to
     provide an equal or higher latency than suggestedLatency wherever possible.
     Actual latency values for an open stream may be retrieved using the
     inputLatency and outputLatency fields of the PaStreamInfo structure
     returned by Pa_GetStreamInfo().
     @see default*Latency in PaDeviceInfo, *Latency in PaStreamInfo
    */
    pub suggestedLatency: PaTime,
    /** An optional pointer to a host api specific data structure
     containing additional information for device setup and/or stream processing.
     hostApiSpecificStreamInfo is never required for correct operation,
     if not used it should be set to NULL.
    */
    pub hostApiSpecificStreamInfo: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_PaStreamParameters() {
    assert_eq!(::std::mem::size_of::<PaStreamParameters>() , 32usize , concat
               ! ( "Size of: " , stringify ! ( PaStreamParameters ) ));
    assert_eq! (::std::mem::align_of::<PaStreamParameters>() , 8usize , concat
                ! ( "Alignment of " , stringify ! ( PaStreamParameters ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaStreamParameters ) ) . device as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( PaStreamParameters ) ,
                "::" , stringify ! ( device ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaStreamParameters ) ) . channelCount as
                * const _ as usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( PaStreamParameters ) ,
                "::" , stringify ! ( channelCount ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaStreamParameters ) ) . sampleFormat as
                * const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( PaStreamParameters ) ,
                "::" , stringify ! ( sampleFormat ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaStreamParameters ) ) . suggestedLatency
                as * const _ as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( PaStreamParameters ) ,
                "::" , stringify ! ( suggestedLatency ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaStreamParameters ) ) .
                hostApiSpecificStreamInfo as * const _ as usize } , 24usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( PaStreamParameters ) ,
                "::" , stringify ! ( hostApiSpecificStreamInfo ) ));
}
impl Clone for PaStreamParameters {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    /** Determine whether it would be possible to open a stream with the specified
 parameters.

 @param inputParameters A structure that describes the input parameters used to
 open a stream. The suggestedLatency field is ignored. See PaStreamParameters
 for a description of these parameters. inputParameters must be NULL for
 output-only streams.

 @param outputParameters A structure that describes the output parameters used
 to open a stream. The suggestedLatency field is ignored. See PaStreamParameters
 for a description of these parameters. outputParameters must be NULL for
 input-only streams.

 @param sampleRate The required sampleRate. For full-duplex streams it is the
 sample rate for both input and output

 @return Returns 0 if the format is supported, and an error code indicating why
 the format is not supported otherwise. The constant paFormatIsSupported is
 provided to compare with the return value for success.

 @see paFormatIsSupported, PaStreamParameters
*/
    pub fn Pa_IsFormatSupported(inputParameters: *const PaStreamParameters,
                                outputParameters: *const PaStreamParameters,
                                sampleRate: f64) -> PaError;
}
/**
 A single PaStream can provide multiple channels of real-time
 streaming audio input and output to a client application. A stream
 provides access to audio hardware represented by one or more
 PaDevices. Depending on the underlying Host API, it may be possible 
 to open multiple streams using the same device, however this behavior 
 is implementation defined. Portable applications should assume that 
 a PaDevice may be simultaneously used by at most one PaStream.

 Pointers to PaStream objects are passed between PortAudio functions that
 operate on streams.

 @see Pa_OpenStream, Pa_OpenDefaultStream, Pa_OpenDefaultStream, Pa_CloseStream,
 Pa_StartStream, Pa_StopStream, Pa_AbortStream, Pa_IsStreamActive,
 Pa_GetStreamTime, Pa_GetStreamCpuLoad

*/
pub type PaStream = ::std::os::raw::c_void;
/** Flags used to control the behavior of a stream. They are passed as
 parameters to Pa_OpenStream or Pa_OpenDefaultStream. Multiple flags may be
 ORed together.

 @see Pa_OpenStream, Pa_OpenDefaultStream
 @see paNoFlag, paClipOff, paDitherOff, paNeverDropInput,
  paPrimeOutputBuffersUsingStreamCallback, paPlatformSpecificFlags
*/
pub type PaStreamFlags = ::std::os::raw::c_ulong;
/**
 Timing information for the buffers passed to the stream callback.

 Time values are expressed in seconds and are synchronised with the time base used by Pa_GetStreamTime() for the associated stream.
 
 @see PaStreamCallback, Pa_GetStreamTime
*/
#[repr(C)]
#[derive(Debug, Copy)]
pub struct PaStreamCallbackTimeInfo {
    /**< The time when the first sample of the input buffer was captured at the ADC input */
    pub inputBufferAdcTime: PaTime,
    /**< The time when the stream callback was invoked */
    pub currentTime: PaTime,
    /**< The time when the first sample of the output buffer will output the DAC */
    pub outputBufferDacTime: PaTime,
}
#[test]
fn bindgen_test_layout_PaStreamCallbackTimeInfo() {
    assert_eq!(::std::mem::size_of::<PaStreamCallbackTimeInfo>() , 24usize ,
               concat ! (
               "Size of: " , stringify ! ( PaStreamCallbackTimeInfo ) ));
    assert_eq! (::std::mem::align_of::<PaStreamCallbackTimeInfo>() , 8usize ,
                concat ! (
                "Alignment of " , stringify ! ( PaStreamCallbackTimeInfo ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaStreamCallbackTimeInfo ) ) .
                inputBufferAdcTime as * const _ as usize } , 0usize , concat !
                (
                "Alignment of field: " , stringify ! (
                PaStreamCallbackTimeInfo ) , "::" , stringify ! (
                inputBufferAdcTime ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaStreamCallbackTimeInfo ) ) .
                currentTime as * const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! (
                PaStreamCallbackTimeInfo ) , "::" , stringify ! ( currentTime
                ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaStreamCallbackTimeInfo ) ) .
                outputBufferDacTime as * const _ as usize } , 16usize , concat
                ! (
                "Alignment of field: " , stringify ! (
                PaStreamCallbackTimeInfo ) , "::" , stringify ! (
                outputBufferDacTime ) ));
}
impl Clone for PaStreamCallbackTimeInfo {
    fn clone(&self) -> Self { *self }
}
/**
 Flag bit constants for the statusFlags to PaStreamCallback.

 @see paInputUnderflow, paInputOverflow, paOutputUnderflow, paOutputOverflow,
 paPrimingOutput
*/
pub type PaStreamCallbackFlags = ::std::os::raw::c_ulong;
//#[repr(u32)]
///**
// Allowable return values for the PaStreamCallback.
// @see PaStreamCallback
//*/
//#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
//pub enum PaStreamCallbackResult {
//    paContinue = 0,
//    paComplete = 1,
//    paAbort = 2,
//}
/**
 Functions of type PaStreamCallback are implemented by PortAudio clients.
 They consume, process or generate audio in response to requests from an
 active PortAudio stream.

 When a stream is running, PortAudio calls the stream callback periodically.
 The callback function is responsible for processing buffers of audio samples 
 passed via the input and output parameters.

 The PortAudio stream callback runs at very high or real-time priority.
 It is required to consistently meet its time deadlines. Do not allocate 
 memory, access the file system, call library functions or call other functions 
 from the stream callback that may block or take an unpredictable amount of
 time to complete.

 In order for a stream to maintain glitch-free operation the callback
 must consume and return audio data faster than it is recorded and/or
 played. PortAudio anticipates that each callback invocation may execute for 
 a duration approaching the duration of frameCount audio frames at the stream 
 sample rate. It is reasonable to expect to be able to utilise 70% or more of
 the available CPU time in the PortAudio callback. However, due to buffer size 
 adaption and other factors, not all host APIs are able to guarantee audio 
 stability under heavy CPU load with arbitrary fixed callback buffer sizes. 
 When high callback CPU utilisation is required the most robust behavior 
 can be achieved by using paFramesPerBufferUnspecified as the 
 Pa_OpenStream() framesPerBuffer parameter.
     
 @param input and @param output are either arrays of interleaved samples or;
 if non-interleaved samples were requested using the paNonInterleaved sample 
 format flag, an array of buffer pointers, one non-interleaved buffer for 
 each channel.

 The format, packing and number of channels used by the buffers are
 determined by parameters to Pa_OpenStream().
     
 @param frameCount The number of sample frames to be processed by
 the stream callback.

 @param timeInfo Timestamps indicating the ADC capture time of the first sample
 in the input buffer, the DAC output time of the first sample in the output buffer
 and the time the callback was invoked. 
 See PaStreamCallbackTimeInfo and Pa_GetStreamTime()

 @param statusFlags Flags indicating whether input and/or output buffers
 have been inserted or will be dropped to overcome underflow or overflow
 conditions.

 @param userData The value of a user supplied pointer passed to
 Pa_OpenStream() intended for storing synthesis data etc.

 @return
 The stream callback should return one of the values in the
 ::PaStreamCallbackResult enumeration. To ensure that the callback continues
 to be called, it should return paContinue (0). Either paComplete or paAbort
 can be returned to finish stream processing, after either of these values is
 returned the callback will not be called again. If paAbort is returned the
 stream will finish as soon as possible. If paComplete is returned, the stream
 will continue until all buffers generated by the callback have been played.
 This may be useful in applications such as soundfile players where a specific
 duration of output is required. However, it is not necessary to utilize this
 mechanism as Pa_StopStream(), Pa_AbortStream() or Pa_CloseStream() can also
 be used to stop the stream. The callback must always fill the entire output
 buffer irrespective of its return value.

 @see Pa_OpenStream, Pa_OpenDefaultStream

 @note With the exception of Pa_GetStreamCpuLoad() it is not permissible to call
 PortAudio API functions from within the stream callback.
*/
pub type PaStreamCallback =
    ::std::option::Option<unsafe extern "C" fn(input:
                                                   *const ::std::os::raw::c_void,
                                               output:
                                                   *mut ::std::os::raw::c_void,
                                               frameCount:
                                                   ::std::os::raw::c_ulong,
                                               timeInfo:
                                                   *const PaStreamCallbackTimeInfo,
                                               statusFlags:
                                                   PaStreamCallbackFlags,
                                               userData:
                                                   *mut ::std::os::raw::c_void)
                              -> ::std::os::raw::c_int>;
extern "C" {
    /** Opens a stream for either input, output or both.
     
 @param stream The address of a PaStream pointer which will receive
 a pointer to the newly opened stream.
     
 @param inputParameters A structure that describes the input parameters used by
 the opened stream. See PaStreamParameters for a description of these parameters.
 inputParameters must be NULL for output-only streams.

 @param outputParameters A structure that describes the output parameters used by
 the opened stream. See PaStreamParameters for a description of these parameters.
 outputParameters must be NULL for input-only streams.
 
 @param sampleRate The desired sampleRate. For full-duplex streams it is the
 sample rate for both input and output
     
 @param framesPerBuffer The number of frames passed to the stream callback
 function, or the preferred block granularity for a blocking read/write stream.
 The special value paFramesPerBufferUnspecified (0) may be used to request that
 the stream callback will receive an optimal (and possibly varying) number of
 frames based on host requirements and the requested latency settings.
 Note: With some host APIs, the use of non-zero framesPerBuffer for a callback
 stream may introduce an additional layer of buffering which could introduce
 additional latency. PortAudio guarantees that the additional latency
 will be kept to the theoretical minimum however, it is strongly recommended
 that a non-zero framesPerBuffer value only be used when your algorithm
 requires a fixed number of frames per stream callback.
 
 @param streamFlags Flags which modify the behavior of the streaming process.
 This parameter may contain a combination of flags ORed together. Some flags may
 only be relevant to certain buffer formats.
     
 @param streamCallback A pointer to a client supplied function that is responsible
 for processing and filling input and output buffers. If this parameter is NULL
 the stream will be opened in 'blocking read/write' mode. In blocking mode,
 the client can receive sample data using Pa_ReadStream and write sample data
 using Pa_WriteStream, the number of samples that may be read or written
 without blocking is returned by Pa_GetStreamReadAvailable and
 Pa_GetStreamWriteAvailable respectively.

 @param userData A client supplied pointer which is passed to the stream callback
 function. It could for example, contain a pointer to instance data necessary
 for processing the audio buffers. This parameter is ignored if streamCallback
 is NULL.
     
 @return
 Upon success Pa_OpenStream() returns paNoError and places a pointer to a
 valid PaStream in the stream argument. The stream is inactive (stopped).
 If a call to Pa_OpenStream() fails, a non-zero error code is returned (see
 PaError for possible error codes) and the value of stream is invalid.

 @see PaStreamParameters, PaStreamCallback, Pa_ReadStream, Pa_WriteStream,
 Pa_GetStreamReadAvailable, Pa_GetStreamWriteAvailable
*/
    pub fn Pa_OpenStream(stream: *mut *mut PaStream,
                         inputParameters: *const PaStreamParameters,
                         outputParameters: *const PaStreamParameters,
                         sampleRate: f64,
                         framesPerBuffer: ::std::os::raw::c_ulong,
                         streamFlags: PaStreamFlags,
                         streamCallback: PaStreamCallback,
                         userData: *mut ::std::os::raw::c_void) -> PaError;
}
extern "C" {
    /** A simplified version of Pa_OpenStream() that opens the default input
 and/or output devices.

 @param stream The address of a PaStream pointer which will receive
 a pointer to the newly opened stream.
 
 @param numInputChannels  The number of channels of sound that will be supplied
 to the stream callback or returned by Pa_ReadStream. It can range from 1 to
 the value of maxInputChannels in the PaDeviceInfo record for the default input
 device. If 0 the stream is opened as an output-only stream.

 @param numOutputChannels The number of channels of sound to be delivered to the
 stream callback or passed to Pa_WriteStream. It can range from 1 to the value
 of maxOutputChannels in the PaDeviceInfo record for the default output device.
 If 0 the stream is opened as an output-only stream.

 @param sampleFormat The sample format of both the input and output buffers
 provided to the callback or passed to and from Pa_ReadStream and Pa_WriteStream.
 sampleFormat may be any of the formats described by the PaSampleFormat
 enumeration.
 
 @param sampleRate Same as Pa_OpenStream parameter of the same name.
 @param framesPerBuffer Same as Pa_OpenStream parameter of the same name.
 @param streamCallback Same as Pa_OpenStream parameter of the same name.
 @param userData Same as Pa_OpenStream parameter of the same name.

 @return As for Pa_OpenStream

 @see Pa_OpenStream, PaStreamCallback
*/
    pub fn Pa_OpenDefaultStream(stream: *mut *mut PaStream,
                                numInputChannels: ::std::os::raw::c_int,
                                numOutputChannels: ::std::os::raw::c_int,
                                sampleFormat: PaSampleFormat, sampleRate: f64,
                                framesPerBuffer: ::std::os::raw::c_ulong,
                                streamCallback: PaStreamCallback,
                                userData: *mut ::std::os::raw::c_void)
     -> PaError;
}
extern "C" {
    /** Closes an audio stream. If the audio stream is active it
 discards any pending buffers as if Pa_AbortStream() had been called.
*/
    pub fn Pa_CloseStream(stream: *mut PaStream) -> PaError;
}
/** Functions of type PaStreamFinishedCallback are implemented by PortAudio 
 clients. They can be registered with a stream using the Pa_SetStreamFinishedCallback
 function. Once registered they are called when the stream becomes inactive
 (ie once a call to Pa_StopStream() will not block).
 A stream will become inactive after the stream callback returns non-zero,
 or when Pa_StopStream or Pa_AbortStream is called. For a stream providing audio
 output, if the stream callback returns paComplete, or Pa_StopStream() is called,
 the stream finished callback will not be called until all generated sample data
 has been played.
 
 @param userData The userData parameter supplied to Pa_OpenStream()

 @see Pa_SetStreamFinishedCallback
*/
pub type PaStreamFinishedCallback =
    ::std::option::Option<unsafe extern "C" fn(userData:
                                                   *mut ::std::os::raw::c_void)>;
extern "C" {
    /** Register a stream finished callback function which will be called when the 
 stream becomes inactive. See the description of PaStreamFinishedCallback for 
 further details about when the callback will be called.

 @param stream a pointer to a PaStream that is in the stopped state - if the
 stream is not stopped, the stream's finished callback will remain unchanged 
 and an error code will be returned.

 @param streamFinishedCallback a pointer to a function with the same signature
 as PaStreamFinishedCallback, that will be called when the stream becomes
 inactive. Passing NULL for this parameter will un-register a previously
 registered stream finished callback function.

 @return on success returns paNoError, otherwise an error code indicating the cause
 of the error.

 @see PaStreamFinishedCallback
*/
    pub fn Pa_SetStreamFinishedCallback(stream: *mut PaStream,
                                        streamFinishedCallback:
                                            PaStreamFinishedCallback)
     -> PaError;
}
extern "C" {
    /** Commences audio processing.
*/
    pub fn Pa_StartStream(stream: *mut PaStream) -> PaError;
}
extern "C" {
    /** Terminates audio processing. It waits until all pending
 audio buffers have been played before it returns.
*/
    pub fn Pa_StopStream(stream: *mut PaStream) -> PaError;
}
extern "C" {
    /** Terminates audio processing immediately without waiting for pending
 buffers to complete.
*/
    pub fn Pa_AbortStream(stream: *mut PaStream) -> PaError;
}
extern "C" {
    /** Determine whether the stream is stopped.
 A stream is considered to be stopped prior to a successful call to
 Pa_StartStream and after a successful call to Pa_StopStream or Pa_AbortStream.
 If a stream callback returns a value other than paContinue the stream is NOT
 considered to be stopped.

 @return Returns one (1) when the stream is stopped, zero (0) when
 the stream is running or, a PaErrorCode (which are always negative) if
 PortAudio is not initialized or an error is encountered.

 @see Pa_StopStream, Pa_AbortStream, Pa_IsStreamActive
*/
    pub fn Pa_IsStreamStopped(stream: *mut PaStream) -> PaError;
}
extern "C" {
    /** Determine whether the stream is active.
 A stream is active after a successful call to Pa_StartStream(), until it
 becomes inactive either as a result of a call to Pa_StopStream() or
 Pa_AbortStream(), or as a result of a return value other than paContinue from
 the stream callback. In the latter case, the stream is considered inactive
 after the last buffer has finished playing.

 @return Returns one (1) when the stream is active (ie playing or recording
 audio), zero (0) when not playing or, a PaErrorCode (which are always negative)
 if PortAudio is not initialized or an error is encountered.

 @see Pa_StopStream, Pa_AbortStream, Pa_IsStreamStopped
*/
    pub fn Pa_IsStreamActive(stream: *mut PaStream) -> PaError;
}
/** A structure containing unchanging information about an open stream.
 @see Pa_GetStreamInfo
*/
#[repr(C)]
#[derive(Debug, Copy)]
pub struct PaStreamInfo {
    /** this is struct version 1 */
    pub structVersion: ::std::os::raw::c_int,
    /** The input latency of the stream in seconds. This value provides the most
     accurate estimate of input latency available to the implementation. It may
     differ significantly from the suggestedLatency value passed to Pa_OpenStream().
     The value of this field will be zero (0.) for output-only streams.
     @see PaTime
    */
    pub inputLatency: PaTime,
    /** The output latency of the stream in seconds. This value provides the most
     accurate estimate of output latency available to the implementation. It may
     differ significantly from the suggestedLatency value passed to Pa_OpenStream().
     The value of this field will be zero (0.) for input-only streams.
     @see PaTime
    */
    pub outputLatency: PaTime,
    /** The sample rate of the stream in Hertz (samples per second). In cases
     where the hardware sample rate is inaccurate and PortAudio is aware of it,
     the value of this field may be different from the sampleRate parameter
     passed to Pa_OpenStream(). If information about the actual hardware sample
     rate is not available, this field will have the same value as the sampleRate
     parameter passed to Pa_OpenStream().
    */
    pub sampleRate: f64,
}
#[test]
fn bindgen_test_layout_PaStreamInfo() {
    assert_eq!(::std::mem::size_of::<PaStreamInfo>() , 32usize , concat ! (
               "Size of: " , stringify ! ( PaStreamInfo ) ));
    assert_eq! (::std::mem::align_of::<PaStreamInfo>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( PaStreamInfo ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaStreamInfo ) ) . structVersion as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( PaStreamInfo ) , "::" ,
                stringify ! ( structVersion ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaStreamInfo ) ) . inputLatency as *
                const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( PaStreamInfo ) , "::" ,
                stringify ! ( inputLatency ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaStreamInfo ) ) . outputLatency as *
                const _ as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( PaStreamInfo ) , "::" ,
                stringify ! ( outputLatency ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const PaStreamInfo ) ) . sampleRate as * const
                _ as usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( PaStreamInfo ) , "::" ,
                stringify ! ( sampleRate ) ));
}
impl Clone for PaStreamInfo {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    /** Retrieve a pointer to a PaStreamInfo structure containing information
 about the specified stream.
 @return A pointer to an immutable PaStreamInfo structure. If the stream
 parameter is invalid, or an error is encountered, the function returns NULL.

 @param stream A pointer to an open stream previously created with Pa_OpenStream.

 @note PortAudio manages the memory referenced by the returned pointer,
 the client must not manipulate or free the memory. The pointer is only
 guaranteed to be valid until the specified stream is closed.

 @see PaStreamInfo
*/
    pub fn Pa_GetStreamInfo(stream: *mut PaStream) -> *const PaStreamInfo;
}
extern "C" {
    /** Returns the current time in seconds for a stream according to the same clock used
 to generate callback PaStreamCallbackTimeInfo timestamps. The time values are
 monotonically increasing and have unspecified origin. 
 
 Pa_GetStreamTime returns valid time values for the entire life of the stream,
 from when the stream is opened until it is closed. Starting and stopping the stream
 does not affect the passage of time returned by Pa_GetStreamTime.

 This time may be used for synchronizing other events to the audio stream, for 
 example synchronizing audio to MIDI.
                                        
 @return The stream's current time in seconds, or 0 if an error occurred.

 @see PaTime, PaStreamCallback, PaStreamCallbackTimeInfo
*/
    pub fn Pa_GetStreamTime(stream: *mut PaStream) -> PaTime;
}
extern "C" {
    /** Retrieve CPU usage information for the specified stream.
 The "CPU Load" is a fraction of total CPU time consumed by a callback stream's
 audio processing routines including, but not limited to the client supplied
 stream callback. This function does not work with blocking read/write streams.

 This function may be called from the stream callback function or the
 application.
     
 @return
 A floating point value, typically between 0.0 and 1.0, where 1.0 indicates
 that the stream callback is consuming the maximum number of CPU cycles possible
 to maintain real-time operation. A value of 0.5 would imply that PortAudio and
 the stream callback was consuming roughly 50% of the available CPU time. The
 return value may exceed 1.0. A value of 0.0 will always be returned for a
 blocking read/write stream, or if an error occurs.
*/
    pub fn Pa_GetStreamCpuLoad(stream: *mut PaStream) -> f64;
}
extern "C" {
    /** Read samples from an input stream. The function doesn't return until
 the entire buffer has been filled - this may involve waiting for the operating
 system to supply the data.

 @param stream A pointer to an open stream previously created with Pa_OpenStream.
 
 @param buffer A pointer to a buffer of sample frames. The buffer contains
 samples in the format specified by the inputParameters->sampleFormat field
 used to open the stream, and the number of channels specified by
 inputParameters->numChannels. If non-interleaved samples were requested using
 the paNonInterleaved sample format flag, buffer is a pointer to the first element 
 of an array of buffer pointers, one non-interleaved buffer for each channel.

 @param frames The number of frames to be read into buffer. This parameter
 is not constrained to a specific range, however high performance applications
 will want to match this parameter to the framesPerBuffer parameter used
 when opening the stream.

 @return On success PaNoError will be returned, or PaInputOverflowed if input
 data was discarded by PortAudio after the previous call and before this call.
*/
    pub fn Pa_ReadStream(stream: *mut PaStream,
                         buffer: *mut ::std::os::raw::c_void,
                         frames: ::std::os::raw::c_ulong) -> PaError;
}
extern "C" {
    /** Write samples to an output stream. This function doesn't return until the
 entire buffer has been written - this may involve waiting for the operating
 system to consume the data.

 @param stream A pointer to an open stream previously created with Pa_OpenStream.

 @param buffer A pointer to a buffer of sample frames. The buffer contains
 samples in the format specified by the outputParameters->sampleFormat field
 used to open the stream, and the number of channels specified by
 outputParameters->numChannels. If non-interleaved samples were requested using
 the paNonInterleaved sample format flag, buffer is a pointer to the first element 
 of an array of buffer pointers, one non-interleaved buffer for each channel.

 @param frames The number of frames to be written from buffer. This parameter
 is not constrained to a specific range, however high performance applications
 will want to match this parameter to the framesPerBuffer parameter used
 when opening the stream.

 @return On success PaNoError will be returned, or paOutputUnderflowed if
 additional output data was inserted after the previous call and before this
 call.
*/
    pub fn Pa_WriteStream(stream: *mut PaStream,
                          buffer: *const ::std::os::raw::c_void,
                          frames: ::std::os::raw::c_ulong) -> PaError;
}
extern "C" {
    /** Retrieve the number of frames that can be read from the stream without
 waiting.

 @return Returns a non-negative value representing the maximum number of frames
 that can be read from the stream without blocking or busy waiting or, a
 PaErrorCode (which are always negative) if PortAudio is not initialized or an
 error is encountered.
*/
    pub fn Pa_GetStreamReadAvailable(stream: *mut PaStream)
     -> ::std::os::raw::c_long;
}
extern "C" {
    /** Retrieve the number of frames that can be written to the stream without
 waiting.

 @return Returns a non-negative value representing the maximum number of frames
 that can be written to the stream without blocking or busy waiting or, a
 PaErrorCode (which are always negative) if PortAudio is not initialized or an
 error is encountered.
*/
    pub fn Pa_GetStreamWriteAvailable(stream: *mut PaStream)
     -> ::std::os::raw::c_long;
}
extern "C" {
    /** Retrieve the size of a given sample format in bytes.

 @return The size in bytes of a single sample in the specified format,
 or paSampleFormatNotSupported if the format is not supported.
*/
    pub fn Pa_GetSampleSize(format: PaSampleFormat) -> PaError;
}
extern "C" {
    /** Put the caller to sleep for at least 'msec' milliseconds. This function is
 provided only as a convenience for authors of portable code (such as the tests
 and examples in the PortAudio distribution.)

 The function may sleep longer than requested so don't rely on this for accurate
 musical timing.
*/
    pub fn Pa_Sleep(msec: ::std::os::raw::c_long);
}
