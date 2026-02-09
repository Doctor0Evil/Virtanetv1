#[derive(Clone, Debug)]
pub struct PendingReview<T> {
    pub payload: T,
}

#[derive(Clone, Debug)]
pub struct ApprovedByHuman<T> {
    pub payload: T,
    pub human_did: String,
}

#[derive(Clone, Debug)]
pub struct Executed<T> {
    pub payload: T,
    pub human_did: String,
    pub hexstamp: String,
}

pub trait HitlState {}
impl<T> HitlState for PendingReview<T> {}
impl<T> HitlState for ApprovedByHuman<T> {}
impl<T> HitlState for Executed<T> {}

impl<T> PendingReview<T> {
    pub fn approve(self, human_did: String) -> ApprovedByHuman<T> {
        ApprovedByHuman { payload: self.payload, human_did }
    }
}

impl<T> ApprovedByHuman<T> {
    pub fn execute<F>(self, log: F) -> Executed<T>
    where
        F: FnOnce(&ApprovedByHuman<T>) -> String, // returns hexstamp from donutloop
    {
        let hexstamp = log(&self);
        Executed {
            payload: self.payload,
            human_did: self.human_did,
            hexstamp,
        }
    }
}
