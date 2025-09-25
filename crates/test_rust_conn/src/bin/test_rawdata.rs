

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    Ok(())
}

/*
use super::*;
    use crate::util::AsyncInlinable;
    #[tokio::test]
    async fn test_raw_data_async_io() {
        use std::io::Cursor;
        let mut buff = Cursor::new(vec![0; 15]);
        const RAW: &[u8] = b"hello rawdata";

        extern "C" fn _empty_free_raw(_raw: raw_data_t) -> i32 {
            0
        }
        let rawdata = RawData::new(
            raw_data_t {
                raw: RAW.as_ptr() as _,
                raw_len: RAW.len() as _,
                raw_type: 0,
            },
            _empty_free_raw,
        );

        let len = AsyncInlinable::write_inlined(&rawdata, &mut buff)
            .await
            .unwrap();
        assert_eq!(len, 19);

        buff.set_position(0);
        let raw: RawData = AsyncInlinable::read_inlined(&mut buff).await.unwrap();
        assert_eq!(raw.raw_len(), RAW.len() as u32);
        assert_eq!(raw.raw_type(), 0);
        assert_eq!(raw.raw_slice(), RAW);
    }
    #[test]
    fn test_raw_data_io() {
        use std::io::Cursor;
        let mut buff = Cursor::new(vec![0; 15]);
        const RAW: &[u8] = b"hello rawdata";

        extern "C" fn _empty_free_raw(_raw: raw_data_t) -> i32 {
            0
        }
        let rawdata = RawData::new(
            raw_data_t {
                raw: RAW.as_ptr() as _,
                raw_len: RAW.len() as _,
                raw_type: 0,
            },
            _empty_free_raw,
        );

        let len = Inlinable::write_inlined(&rawdata, &mut buff).unwrap();
        assert_eq!(len, 19);

        buff.set_position(0);
        let raw: RawData = Inlinable::read_inlined(&mut buff).unwrap();
        assert_eq!(raw.raw_len(), RAW.len() as u32);
        assert_eq!(raw.raw_type(), 0);
        assert_eq!(raw.raw_slice(), RAW);
    }
 */