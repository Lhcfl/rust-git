
use crate::{
    models::{
        commit::{Commit, CommitBuilder},
        object::{Object, Sha1Able},
        repo::Repository,
    },
    services::{object::ObjectService, tree::compare_trees},
};

use super::tree::ComparedLine;

pub struct CommitCreationInfo {
    pub compared: Option<Vec<ComparedLine>>,
    pub commit_sha1: String,
    pub is_root: bool,
    pub branch_name: String,
}

pub enum CommitCreateResult {
    Success(CommitCreationInfo),
    NothingToCommit,
}

pub trait CommitService {
    fn create_commit(&self, message: &str) -> anyhow::Result<CommitCreateResult>;
}

impl CommitService for Repository {
    fn create_commit(&self, message: &str) -> anyhow::Result<CommitCreateResult> {
        let (branch, is_new) = self.head().load_branch_or_create()?;
        let message = message.to_owned();

        // Step 2: Generate tree object from the stage
        let stage = self.stage()?;
        let tree = stage.map(|s| s.0);

        // Step 3: Compare the tree with the current HEAD, to check if working tree clean
        let compared = if !is_new {
            let current_commit = self.load_object(&branch.head)?.map(|o| o.cast_commit());
            let current_tree = current_commit.get_tree()?;

            let compared = compare_trees(&current_tree, &tree)?;
            if compared.is_empty() {
                return Ok(CommitCreateResult::NothingToCommit);
            }
            Some(compared)
        } else {
            None
        };

        // Step 4: Create commit object
        let tree = tree.map(Object::Tree);
        tree.save()?;
        let commit = Commit::new(CommitBuilder {
            tree: tree.sha1().into(),
            parent: if is_new {
                None
            } else {
                Some(branch.head.clone())
            },
            message: message.to_string(),
        });

        let commit_sha1 = commit.sha1();
        self.wrap(Object::Commit(commit)).save()?;

        // Step 5: Update branch HEAD
        let mut branch_cloned = branch.cloned();
        branch_cloned.head = commit_sha1.clone().into();
        branch_cloned.save()?;

        Ok(CommitCreateResult::Success(CommitCreationInfo {
            compared,
            commit_sha1,
            branch_name: branch.full_name(),
            is_root: is_new,
        }))
    }
}
