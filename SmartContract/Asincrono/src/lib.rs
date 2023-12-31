#![no_std]
use gmeta::Metadata;
use hashbrown::HashMap;
use io::*;
use gstd::{async_main, msg, exec, prelude::*, ActorId};

#[derive(Debug, Clone, Default)]
struct Actors {  
    actors: HashMap<ActorId, u128>,
}

impl Actors {

    async fn compra( &mut self, amount_tokens: u128){

        let currentstate = state_mut();

        let address_ft = addresft_state_mut();

        let payload = FTAction::Burn(amount_tokens);
     
        let result =  msg::send_for_reply_as::<_, FTEvent>(address_ft.ft_program_id,payload,0,0).expect("Error in sending a message").await;
        
        currentstate.entry(msg::source()).or_insert(amount_tokens); 

        let _ = match result {
            Ok(event) => match event {
                FTEvent::Ok => Ok(()),
                _ => Err(()),
            },
            Err(_) => Err(()),
        };
    }

    async fn puntaje(&mut self, amount_tokens: u128){

        let currentstate = state_mut();
        let address_ft = addresft_state_mut();           
        let payload = FTAction::Mint(amount_tokens);     
        let result =  msg::send_for_reply_as::<_, FTEvent>(address_ft.ft_program_id,payload,0,0).expect("Error in sending a message").await;
        currentstate.entry(msg::source()).or_insert(amount_tokens);  

        let _ = match result {
            Ok(event) => match event {
                FTEvent::Ok => Ok(()),
                _ => Err(()),
            },
            Err(_) => Err(()),
        };
    }

    async fn regalo(&mut self, amount_tokens: u128) {
 
        let currentstate = state_mut();
        let address_ft = addresft_state_mut();           
        let payload = FTAction::Transfer{from: exec::program_id(), to: msg::source() ,amount: amount_tokens};
        let _ = msg::send(address_ft.ft_program_id, payload, 0);
        currentstate.entry(msg::source()).or_insert(amount_tokens);  
       


    }

   
}

// Definimos las varibles.
static mut ACTORS:Option<Actors> = None;

static mut STATE:Option<HashMap<ActorId, u128>> = None;

static mut ADDRESSFT:Option<InitFT> = None;

fn actors_state_mut() -> &'static mut Actors  {

    unsafe { ACTORS.get_or_insert(Default::default()) }


}

fn state_mut() -> &'static mut HashMap<ActorId,u128> {

    let state = unsafe { STATE.as_mut()};

    unsafe { state.unwrap_unchecked() }


}

fn addresft_state_mut() -> &'static mut InitFT {


    let addressft = unsafe { ADDRESSFT.as_mut()};

    unsafe { addressft.unwrap_unchecked() }


}

#[no_mangle]
extern "C" fn init () {

    let config: InitFT = msg::load().expect("Unable to decode InitFT");

    let _actors = Actors {
        ..Default::default()
    };

    if config.ft_program_id.is_zero() {
        panic!("FT program address can't be 0");
    }

    let initft = InitFT {
        ft_program_id: config.ft_program_id
    };

    unsafe {
        ADDRESSFT = Some(initft);
    }

   unsafe { STATE = Some(HashMap::new())}

}

#[async_main]
async fn main(){

    let action: Action = msg::load().expect("Could not load Action");

    let actors = unsafe { ACTORS.get_or_insert(Actors::default()) };

    match action {
        Action::FTPuntaje(amount) =>  {
         

                actors.puntaje(amount).await;
               
 
            },
        Action::FTCompra(amount) => {

                
                actors.compra(amount).await;
                     
            }

        Action::FTRegalo(amount) => {
     
                actors.regalo(amount).await;
                
             
            }
           
            };

}

 
#[no_mangle]
extern "C" fn state() {
 
    let state: <ContractMetadata as Metadata>::State =
        state_mut().iter().map(|(k, v)| (*k, *v)).collect();
     
    msg::reply(state, 0).expect("failed to encode or reply from `state()`");
}
