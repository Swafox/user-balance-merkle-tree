use user_balance_merkle_tree::MerkleTree;

#[test]
fn test_add_balance() {
    let mut tree = MerkleTree::new();
    tree.add_balance("user1".to_string(), 100);
    assert_eq!(tree.get_balance("user1"), Some(100));
}

#[test]
fn test_get_root() {
    let mut tree: MerkleTree = MerkleTree::new();
    tree.add_balance("user1".to_string(), 100);
    tree.add_balance("user2".to_string(), 200);
    assert!(tree.get_root().is_some());
}

#[test]
fn test_multiple_balances() {
    let mut tree: MerkleTree = MerkleTree::new();
    tree.add_balance("user1".to_string(), 100);
    tree.add_balance("user2".to_string(), 200);
    tree.add_balance("user3".to_string(), 300);
    assert_eq!(tree.get_balance("user1"), Some(100));
    assert_eq!(tree.get_balance("user2"), Some(200));
    assert_eq!(tree.get_balance("user3"), Some(300));
}

#[test]
fn test_nonexistent_balance() {
    let tree = MerkleTree::new();
    assert_eq!(tree.get_balance("nonexistent"), None);
}

#[test]
fn test_tree_size() {
    let mut tree = MerkleTree::new();
    tree.add_balance("user1".to_string(), 100);
    tree.add_balance("user2".to_string(), 200);
    assert!(tree.size_in_bytes() > 0);
    assert!(tree.size_in_kb() > 0.0);
    assert!(tree.size_in_mb() > 0.0);
}