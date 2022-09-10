use super::{
    Cerebrum, Engine,
    event::{AccountEvent, Command, Event},
    market::MarketUpdater,
    account::AccountUpdater,
    command::Commander,
};
use barter_data::model::MarketEvent;

/// Consumer can transition to one of:
///  a) MarketUpdater
///  b) AccountUpdater
///  c) Commander
pub struct Consumer;

impl<Strategy> Cerebrum<Consumer, Strategy> {
    pub fn next_event(mut self) -> Engine<Strategy> {
        // Consume next Event
        match self.feed.next() {
            Event::Market(market) => {
                Engine::MarketUpdater(Cerebrum::from((self, market)))
            }
            Event::Account(account) => {
                Engine::AccountUpdater(Cerebrum::from((self, account)))
            }
            Event::Command(command) => {
                Engine::Commander(Cerebrum::from((self, command)))
            }
        }
    }
}

/// a) Consumer -> MarketUpdater
impl<Strategy> From<(Cerebrum<Consumer, Strategy>, MarketEvent)> for Cerebrum<MarketUpdater, Strategy> {
    fn from((cerebrum, market): (Cerebrum<Consumer, Strategy>, MarketEvent)) -> Self {
        Self {
            state: MarketUpdater { market },
            feed: cerebrum.feed,
            accounts: cerebrum.accounts,
            exchange_tx: cerebrum.exchange_tx,
            strategy: cerebrum.strategy,
            event_tx: cerebrum.event_tx,
        }
    }
}

/// b) Consumer -> AccountUpdater
impl<Strategy> From<(Cerebrum<Consumer, Strategy>, AccountEvent)> for Cerebrum<AccountUpdater, Strategy> {
    fn from((cerebrum, account): (Cerebrum<Consumer, Strategy>, AccountEvent)) -> Self {
        Self {
            state: AccountUpdater { account },
            feed: cerebrum.feed,
            accounts: cerebrum.accounts,
            exchange_tx: cerebrum.exchange_tx,
            strategy: cerebrum.strategy,
            event_tx: cerebrum.event_tx,
        }
    }
}

/// c) Consumer -> Commander
impl<Strategy> From<(Cerebrum<Consumer, Strategy>, Command)> for Cerebrum<Commander, Strategy> {
    fn from((cerebrum, command): (Cerebrum<Consumer, Strategy>, Command)) -> Self {
        Self {
            state: Commander { command },
            feed: cerebrum.feed,
            accounts: cerebrum.accounts,
            exchange_tx: cerebrum.exchange_tx,
            strategy: cerebrum.strategy,
            event_tx: cerebrum.event_tx,
        }
    }
}