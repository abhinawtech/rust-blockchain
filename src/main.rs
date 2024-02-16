
#[derive(Debug, Clone)]
struct Blockchain{
    block:Vec<Block>
}

#[derive(Debug, Clone)]
struct Block{
    id:u64,
    nonce: u64,
    data: String,
    hash: String,
    previous_hash: String,
    timestamp: i64
}

impl Blockchain {
    fn new()->Self{
        Self { block: vec![] }
    }

    fn starting_block(&mut self){
        let genesis_block = Block{
            id:1,
            nonce: 11316,
            data:String::from("I am a first or  genesis block"),
            hash: String::from("000015783b"),
            previous_hash: String::from("000000000000000000000000000"),
            timestamp:Utc::now().timestamp()

        };
        self.block.push(genesis_block);
    }

    fn try_add_block(&mut self, block: Block){
        match self.block.last() {
            None =>{
                println!("The blockchain does not have atleast one block");
                return;
            }
            Some(latest_block)=>{
                if self.is_block_valid(&block,latest_block){
                    self.block.push(block);
                    println!("Block has been successfully added!");
                }
                else {
                    println!("could not add the block");
                }
            }
        }
       
    }
    fn is_block_valid(&self, new_block: &Block, latest_block: &Block)->bool{
        if new_block.previous_hash != latest_block.hash{
            println!("Block with id {:?} has wrong previous hash",new_block);
            return false;
        }
        else if !new_block.hash.starts_with("0000") {
            println!("Block with id: {} has invalid hash",new_block.id);
            return false;
        }
        else if new_block.id != latest_block.id+1 {
            println!("Block with id {} is not the next block after the latest block with id: {}",new_block.id, latest_block.id);
            return false;
        }
        else if digest(format!("{}{}{}{}{}",new_block.id,&new_block.previous_hash,&new_block.data,new_block.timestamp,new_block.nonce))!=new_block.hash {
            println!("block with id {} has invalid hash",new_block.id );
            return  false;
        }
        true
    }
    fn is_chain_valid(&self, chain:&Vec<Block>)->bool{
        match chain.len() {
            0=> println!("The chain is empty"),
            1=> println!("The chain only contains 1 block"),
            _=> {
                for i in 1..chain.len(){
                    let previous = chain.get(i-1).unwrap();
                    let current = chain.get(i).unwrap();
                    if !self.is_block_valid(current, previous){
                        return false;
                    }
                }
            }
        }
        println!("The chain found to be correct");
        true
    }
}

impl Block {
    fn new(id: u64, previous_hash: String, data: String)->Self{
        let now = Utc::now();
        let now_timestamp = now.timestamp();

        let(nonce, hash) = Block::mine_block(id, now_timestamp, &previous_hash, &data);

        Self { id,
                 nonce,
                  data,
                   hash,
                    previous_hash,
                     timestamp:now_timestamp }
    }

    fn mine_block(id: u64, timestamp: i64, previous_hash: &str, data: &str)->(u64, String){
        println!("mining in progress");
        let mut nonce = 1;

        loop{
            let block_string = format!("{}{}{}{}{}",id, previous_hash,data,timestamp,nonce);
            let hash = digest(block_string);    
            if hash.starts_with("0000"){
                println!("mined! nonce: {}, hash: {}",nonce, hash);  
                return (nonce, hash);
            }

            nonce +=1;
        }

    }


    
}
use core::time;
use sha256::digest;
use chrono::Utc;
fn main() {
    let mut new_BC = Blockchain::new();
    new_BC.starting_block();
    println!("{:?}",new_BC);

    let new_block = Block::new(2, new_BC.block[0].hash.to_owned(), "Abhinaw".to_string());

    new_BC.try_add_block(new_block);

    let new_block = Block::new(3, new_BC.block[1].hash.to_owned(), "Abhinaw3".to_string());

    new_BC.try_add_block(new_block);
    let new_block = Block::new(4, new_BC.block[2].hash.to_owned(), "Abhinaw4".to_string());

    new_BC.try_add_block(new_block);
    let new_block = Block::new(5, new_BC.block[3].hash.to_owned(), "Abhina5".to_string());

    new_BC.try_add_block(new_block);
    let new_block = Block::new(6, new_BC.block[4].hash.to_owned(), "Abhinaw6".to_string());

    new_BC.try_add_block(new_block);

    new_BC.is_chain_valid(&new_BC.block);

}
