use std::ptr;

use super::Context;
use ffi::*;
use libc::c_int;
use {Error, Frame};

pub struct Source<'a> {
    ctx: &'a mut Context<'a>,
}

impl<'a> Source<'a> {
    pub unsafe fn wrap<'b>(ctx: &'b mut Context<'b>) -> Source<'b> {
        Source { ctx }
    }
}

impl<'a> Source<'a> {
    pub fn failed_requests(&self) -> usize {
        unsafe { av_buffersrc_get_nb_failed_requests(self.ctx.as_ptr() as *mut _) as usize }
    }

    pub fn add(&mut self, frame: &Frame) -> Result<(), Error> {
        unsafe {
            match av_buffersrc_add_frame(self.ctx.as_mut_ptr(), frame.as_ptr() as *mut _) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }
    pub fn add_flags(&mut self, frame: &Frame, flags: Flags) -> Result<(), Error> {
        unsafe {
            match av_buffersrc_add_frame_flags(
                self.ctx.as_mut_ptr(),
                frame.as_ptr() as *mut _,
                flags.bits(),
            ) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        unsafe { self.add(&Frame::wrap(ptr::null_mut())) }
    }

    pub fn close(&mut self, pts: i64) -> Result<(), Error> {
        unsafe {
            match av_buffersrc_close(self.ctx.as_mut_ptr(), pts, 0) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }
}

bitflags! {
    // No clue why these have to be cast to c_int but the buffersink ones don't.
    // Probably should figure it out but oh well
    pub struct Flags: c_int {
        const NO_CHECK_FORMAT = AV_BUFFERSRC_FLAG_NO_CHECK_FORMAT as c_int;
        const PUSH = AV_BUFFERSRC_FLAG_PUSH as c_int;
        const KEEP_REF = AV_BUFFERSRC_FLAG_KEEP_REF as c_int;
    }
}
