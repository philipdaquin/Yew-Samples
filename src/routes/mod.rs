pub mod counter;
pub mod home;

use yew_router::prelude::*;

#[derive(Debug, Switch, Clone)]
pub enum AppRoute { 
    #[to = "/counter"]
    Counter,
    #[to = "/"]
    Home,
   
}