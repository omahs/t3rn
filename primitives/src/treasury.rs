use crate::common::RoundInfo;

pub trait Treasury<T: frame_system::Config>
{
    type Error;

    fn current_round() -> RoundInfo<T::BlockNumber>;
}