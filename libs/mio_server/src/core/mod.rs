


pub struct CompositeFile {
    data: ReadOnlySource,
    offsets_index: HashMap<FileAddr, (usize, uszie)>,
}
impl CompositeFile {
    pub fn open(data: &ReadOnlySource) -> io::Result<CompositeFile> {
        let end = data.len();
        let footer_len_data = data.slice_from(end-4);
        let footer_len = u32::deserializer(&mut footer_len_data.as_slice())? as usize;
        let footer_start = end - 4 - footer_len;
        let footer_data = data.slice(footer_start, footer_start + footer_len);
        let mut footer_buffer = footer_data.as_slice();
        let num_fields = VInt::deserialize(&mut footer_buffer)?.0 as usize;
        let mut file_addrs = vec![];
        let mut offsets = vec![];
        let mut field_index = HashMap::new();
        let mut offset = 0;
        for _ in 0..num_fields {
            offset += VInt::deserialize(&mut footer_buffer)?.0 as usize;
            let file_addr = FileAddr::deserialize(&mut footer_buffer)?;
            offsets.push(offset);
            file_addrs.push(file_addr);
        }
        offsets.push(footer_start);
        for i in 0..num_fields {
            let file_addr = file_addrs[i];
            let start_offset = offsets[i];
            let end_offset = offsets[i + 1];
            field_index.insert(file_addr, (start_offset, end_offset));
        }
        Ok(CompositeFile {
            data: data.slice_to(footer_start),
            offsets_index: field_index,
        })
    }

}
