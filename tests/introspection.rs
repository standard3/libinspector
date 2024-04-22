#[cfg(test)]
mod instrospection_tests {
    use std::{path::Path, str::FromStr};

    use libinspector::introspection::segment::*;

    #[test]
    fn test_create_empty_segment_from_str() {
        let segment = Segment::from_str(
            "0000000000000000-0000000000000000 ---- 0000000000000000 00:00 0 /dev/null",
        )
        .unwrap();

        assert_eq!(segment.start, 0);
        assert_eq!(segment.end, 0);
        assert_eq!(segment.permissions, [SegmentPermission::NoPermission; 4]);
        assert_eq!(segment.offset, 0);
        assert_eq!(segment.device, Device::new(0, 0));
        assert_eq!(segment.inode, 0);
        assert_eq!(
            segment.pathname,
            SegmentType::Code(Box::new(Path::new("/dev/null").to_owned()))
        );
    }

    #[test]
    fn test_create_stack_segment_from_str() {
        let segment = Segment::from_str(
            "7ffea490d000-7ffea4a0f000 rw-p 00000000 00:00 0                          [stack]",
        )
        .unwrap();

        assert_eq!(segment.start, 0x7ffea490d000);
        assert_eq!(segment.end, 0x7ffea4a0f000);
        assert_eq!(
            segment.permissions,
            [
                SegmentPermission::Read,
                SegmentPermission::Write,
                SegmentPermission::NoPermission,
                SegmentPermission::Private,
            ]
        );
        assert_eq!(segment.offset, 0);
        assert_eq!(segment.device, Device::new(0, 0));
        assert_eq!(segment.inode, 0);
        assert_eq!(segment.pathname, SegmentType::Stack);
    }
}
