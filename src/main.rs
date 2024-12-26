mod rpc;
mod ui;

use anyhow::Result;
use crate::ui::UI;

fn main() -> Result<()> {
    println!("Bitcoin Node Terminal UI starting...");
    
    let mut ui = UI::new()?;
    ui.run()?;
    
    Ok(())
}
