// The MIT License (MIT)
//
// Copyright (c) 2013 Jeremy Letang (letang.jeremy@gmail.com)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

/*!
* The MAC_CORE specific API.
*/

use pa::*;
use types::*;
use ffi;

pub static PaMacCoreChangeDeviceParameters : u32 = 0x01;
pub static PaMacCoreFailIfConversionRequired : u32 = 0x02;
pub static PaMacCoreConversionQualityMin : u32 = 0x0100;
pub static PaMacCoreConversionQualityMedium : u32 = 0x0200;
pub static PaMacCoreConversionQualityLow : u32 = 0x0300;
pub static PaMacCoreConversionQualityHigh : u32 = 0x0400;
pub static PaMacCoreConversionQualityMax : u32 = 0x0000;
pub static PaMacCorePlayNice : u32 = 0x00;
pub static PaMacCorePro : u32 = 0x01;
pub static PaMacCoreMinimizeCPUButPlayNice : u32 = 0x0100;
pub static PaMacCoreMinimizeCPU : u32 = 0x0101;


/// Not implemented
pub struct PaMacCoreStreamInfo {
    size : u32,
    host_api_type : PaHostApiTypeId,
    version : u32,
    flags : u32,
    channel_map : *i32,
    channel_map_size : u32    
}

pub trait MacCore {
    fn get_stream_input_device(&self) -> PaDeviceIndex;
    fn get_stream_output_device(&self) -> PaDeviceIndex; 
}

// // fn get_buffer_size_range(device : PaDeviceIndex) -> Result<(u32, u32), PaError> {
//     let mut min_buffer_size_frames : u32 = 0;
//     let mut max_buffer_size_frames : u32 = 0;
//     let err = unsafe { ffi::PaMacCore_GetBufferSizeRange(device, &min_buffer_size_frames, &max_buffer_size_frames) };
//     match err {
//         PaNoError   => Ok((min_buffer_size_frames, max_buffer_size_frames)),
//         _           => Err(err)
//     }
// }


impl<S> MacCore for PaStream<S> {
        fn get_stream_input_device(&self) -> PaDeviceIndex {
        unsafe {
            ffi::PaMacCore_GetStreamInputDevice(self.get_c_pa_stream())
        }
    }
        fn get_stream_output_device(&self) -> PaDeviceIndex {
        unsafe {
            ffi::PaMacCore_GetStreamOutputDevice(self.get_c_pa_stream())
        }
    }
}