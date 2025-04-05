use alloy_sol_types::sol;

sol! {
    #[derive(Debug, PartialEq)]
    interface WETH9 {
        function withdraw(uint wad) public;
    }
}
