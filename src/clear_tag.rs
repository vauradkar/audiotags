use id3::Timestamp;

use crate::{
    Album, AudioTag, AudioTagConfig, AudioTagEdit, AudioTagWrite, FlacTag, Id3v2Tag, Mp4Tag,
    Picture, Result, ToAny, ToAnyTag,
};

/// A enum that contains all tag formats as variants. This helps to have
/// finer control over the internal structs.
/// The `From<AnyTag<'a>>` and `From<Box<dyn AudioTag + Send + Sync>>` are
/// lossy coversions.
pub enum ClearTag {
    Id3(Id3v2Tag),
    Flac(FlacTag),
    Mp4(Mp4Tag),
}

/// Generates helper function to get inner types
macro_rules! make_get_inner {
    ($op:ident, $clear_tag:ty, $var:expr, $getter:ident, $ret:ty) => {
        paste::item! {
            pub fn $op(self: $clear_tag) -> $ret {
                if let $var(v) = self {
                    Some(v.$getter())
                } else {
                    None
                }
            }
        }
    };
}

/// Generates helper function to get inner types
macro_rules! make_get_inners {
    ($op:ident, $clear_tag:ty, $var:expr, $ret:ty) => {
        paste::item! {
        make_get_inner!(
            [< get_inner_ $op >],
            & $clear_tag,
            $var,
            inner_ref,
            Option<& $ret>
        );
        make_get_inner!(
            [< get_inner_ $op _mut >],
            &mut $clear_tag,
            $var,
            inner_mut_ref,
            Option<&mut $ret>
        );
        }
    };
}

impl ClearTag {
    make_get_inners!(flac, Self, Self::Flac, metaflac::Tag);
    make_get_inners!(mp4, Self, Self::Mp4, mp4ameta::Tag);
    make_get_inners!(id3, Self, Self::Id3, id3::Tag);
}

/// Generates helper function to get or remove tag
macro_rules! make_op {
    ($op:ident, $clear_tag:ty, $ret:ty) => {
        paste::item! {
            fn $op(self: $clear_tag) -> $ret {
                match self {
                    ClearTag::Id3(id3) => id3.$op(),
                    ClearTag::Flac(flac) => flac.$op(),
                    ClearTag::Mp4(mp4) => mp4.$op(),
                }
            }
        }
    };
}

/// Generates helper function to update tag with return type
macro_rules! make_update_op_ret {
    ($update:ident, $clear_tag:ty, $arg_type:ty, $ret_type:ty) => {
        paste::item! {
            fn $update(self: $clear_tag, arg: $arg_type) -> $ret_type {
                match self {
                    ClearTag::Id3(id3) => id3.$update(arg),
                    ClearTag::Flac(flac) => flac.$update(arg),
                    ClearTag::Mp4(mp4) => mp4.$update(arg),
                }
            }
        }
    };
}

/// Generates helper function to update tag without return type
macro_rules! make_update_op {
    ($update:ident, $clear_tag:ty, $arg_type:ty) => {
        paste::item! {
            make_update_op_ret!($update, $clear_tag, $arg_type, ());
        }
    };
}

/// Generates get, remove and set functions
macro_rules! make_get_set_remove {
    ($op:ident, $clear_tag:ty, $ret:ty) => {
        paste::item! {
            make_op!($op, & $clear_tag, Option<$ret>);
            make_update_op!([< set_ $op >], &mut $clear_tag, $ret);
            make_op!([< remove_ $op >], &mut $clear_tag, ());
        }
    };
}

impl AudioTagEdit for ClearTag {
    make_get_set_remove!(title, Self, &str);
    make_get_set_remove!(artist, Self, &str);
    make_get_set_remove!(date, Self, Timestamp);
    make_get_set_remove!(year, Self, i32);
    make_get_set_remove!(album_title, Self, &str);
    make_get_set_remove!(album_artist, Self, &str);
    make_get_set_remove!(album_cover, Self, Picture);
    make_get_set_remove!(composer, Self, &str);
    make_get_set_remove!(track_number, Self, u16);
    make_get_set_remove!(total_tracks, Self, u16);
    make_get_set_remove!(disc_number, Self, u16);
    make_get_set_remove!(total_discs, Self, u16);
    make_get_set_remove!(genre, Self, &str);
    make_get_set_remove!(comment, Self, &str);
    make_get_set_remove!(lyricist, Self, &str);
    make_get_set_remove!(album, Self, Album);
    make_op!(duration, &Self, Option<f64>);
    make_op!(artists, &Self, Option<Vec<&str>>);
    make_op!(album_artists, &Self, Option<Vec<&str>>);
    make_update_op!(add_artist, &mut Self, &str);
    make_update_op!(add_album_artist, &mut Self, &str);
}

impl AudioTagWrite for ClearTag {
    make_update_op_ret!(write_to, &mut Self, &mut std::fs::File, Result<()>);
    make_update_op_ret!(write_to_path, &mut Self, &str, Result<()>);
}

impl ToAny for ClearTag {
    make_op!(to_any, &Self, &dyn std::any::Any);
    make_op!(to_any_mut, &mut Self, &mut dyn std::any::Any);
}

impl ToAnyTag for ClearTag {
    make_op!(to_anytag, &Self, crate::AnyTag<'_>);
}

impl AudioTagConfig for ClearTag {
    make_op!(config, &Self, &crate::Config);
    make_update_op!(set_config, &mut Self, crate::Config);
}

impl AudioTag for ClearTag {}
