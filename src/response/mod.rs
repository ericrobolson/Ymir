use core::{fmt::Write, panic, unimplemented};

use crate::parser::StreamError;
mod status;
pub use status::Status;
mod content_type;
pub use content_type::ContentType;

const HEADER_COUNT: usize = 100;

#[derive(Copy, Clone, Debug)]
pub struct Header {
    pub key: &'static str,
    pub value: &'static str,
}

#[derive(Copy, Clone, Debug)]
pub enum ResponseError {
    HeaderLengthExceeded { max_headers: usize },
}

pub struct Response<'a> {
    status: Status,
    body: &'a [u8],
    headers: [Option<Header>; HEADER_COUNT],
}

impl<'a> Response<'a> {
    pub fn new(status: Status) -> Self {
        Self {
            status,
            body: &[],
            headers: [None; HEADER_COUNT],
        }
    }

    pub fn with_body(&mut self, body: &'a [u8]) -> Result<(), ResponseError> {
        self.body = body;
        Ok(())
    }

    pub fn with_header(&mut self, header: Header) -> Result<(), ResponseError> {
        let mut i = None;

        for j in 0..HEADER_COUNT {
            if self.headers[j].is_none() {
                i = Some(j);
            }
        }

        match i {
            Some(i) => {
                self.headers[i] = Some(header);
            }
            None => {
                return Err(ResponseError::HeaderLengthExceeded {
                    max_headers: HEADER_COUNT,
                })
            }
        }

        Ok(())
    }

    fn version() -> &'static str {
        "HTTP/1.1"
    }

    fn newline() -> &'static str {
        "\r\n"
    }

    fn space() -> &'static str {
        " "
    }

    /// Returns the size of the buffer that should be used for writing the response.
    pub fn request_size(&self) -> usize {
        let mut byte_index = 0;

        // Execute the write passes, calculating the total size
        for pass in Self::write_passes() {
            let size = self.execute_write_pass(pass, 0, &mut []);
            byte_index += size;
        }

        byte_index
    }

    fn write_passes<'b>() -> &'b [WritePass<'b>] {
        &[
            WritePass::WriteVersion,
            WritePass::WriteSpace,
            WritePass::WriteStatus,
            WritePass::WriteNewline,
            WritePass::WriteHeaders,
            WritePass::WriteNewline,
            WritePass::WriteNewline,
            WritePass::WriteBody,
        ]
    }

    /// Executes a write pass. In the event that an empty buffer is passed, will not write to the buffer but will instead calculate the size that would be returned.
    fn execute_write_pass(&self, pass: &WritePass, start_index: usize, buffer: &mut [u8]) -> usize {
        let skip_buffering = buffer.len() == 0;

        // TODO: stack overflow stuff? Not worrying for now...

        let mut byte_index = start_index;
        let mut data_to_copy: &[u8] = &[];

        match pass {
            WritePass::WriteVersion => {
                data_to_copy = &Self::version().as_bytes();
            }
            WritePass::WriteStatus => {
                data_to_copy = &self.status.to_status().as_bytes();
            }
            WritePass::WriteHeaders => {
                // Write content length
                {
                    use numtoa::NumToA;
                    let mut test_buff: [u8; 20] = [0; 20];
                    let len = self.body.len().numtoa_str(10, &mut test_buff);

                    byte_index += self.execute_write_pass(
                        &WritePass::WriteHeader {
                            key: "Content-Length".as_bytes(),
                            value: len.as_bytes(),
                        },
                        byte_index,
                        buffer,
                    );
                }

                // Write other headers
                for header in &self.headers {
                    match header {
                        Some(header) => {
                            byte_index += self.execute_write_pass(
                                &WritePass::WriteHeader {
                                    key: &header.key.as_bytes(),
                                    value: &header.value.as_bytes(),
                                },
                                byte_index,
                                buffer,
                            );
                        }
                        None => {}
                    }
                }
            }

            WritePass::WriteNewline => {
                data_to_copy = &Self::newline().as_bytes();
            }
            WritePass::WriteBody => {
                data_to_copy = &self.body;
            }
            WritePass::WriteSpace => {
                data_to_copy = &Self::space().as_bytes();
            }
            WritePass::WriteBytes(bytes) => {
                data_to_copy = bytes;
            }
            WritePass::WriteHeader { key, value } => {
                // Write key
                byte_index +=
                    self.execute_write_pass(&WritePass::WriteBytes(key), byte_index, buffer);

                // Write separator
                byte_index += self.execute_write_pass(
                    &WritePass::WriteBytes(":".as_bytes()),
                    byte_index,
                    buffer,
                );

                // Write space
                byte_index += self.execute_write_pass(&WritePass::WriteSpace, byte_index, buffer);

                // Write value
                byte_index +=
                    self.execute_write_pass(&WritePass::WriteBytes(value), byte_index, buffer);

                // Write newline
                byte_index += self.execute_write_pass(&WritePass::WriteNewline, byte_index, buffer);
            }
        }

        // Write the data to the buffer
        if data_to_copy.len() > 0 {
            for byte in data_to_copy {
                if !skip_buffering {
                    buffer[byte_index] = *byte;
                }
                byte_index += 1;
            }
        }

        // Return the size of the written data
        return byte_index - start_index;
    }

    /// Writes the response and body to the buffer, returning the number of written bytes.
    pub fn write_to_buffer(&self, buffer: &mut [u8]) -> Result<usize, StreamError> {
        if buffer.len() < self.request_size() {
            return Err(StreamError::BufferOverflow {
                max_bytes: buffer.len(),
                actual_size: self.request_size(),
            });
        }

        let mut byte_index = 0;

        // Execute the write passes, adding it to the buffer
        for pass in Self::write_passes() {
            let size = self.execute_write_pass(pass, 0, &mut buffer[byte_index..]);
            byte_index += size;
        }

        Ok(byte_index)
    }
}

enum WritePass<'a> {
    WriteVersion,
    WriteStatus,
    WriteHeaders,
    WriteHeader { key: &'a [u8], value: &'a [u8] },
    WriteBytes(&'a [u8]),
    WriteBody,
    WriteNewline,
    WriteSpace,
}

#[cfg(test)]
mod tests {
    use super::*;
}
