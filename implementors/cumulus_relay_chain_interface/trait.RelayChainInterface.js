(function() {var implementors = {
"cumulus_relay_chain_inprocess_interface":[["impl&lt;Client&gt; <a class=\"trait\" href=\"cumulus_relay_chain_interface/trait.RelayChainInterface.html\" title=\"trait cumulus_relay_chain_interface::RelayChainInterface\">RelayChainInterface</a> for <a class=\"struct\" href=\"cumulus_relay_chain_inprocess_interface/struct.RelayChainInProcessInterface.html\" title=\"struct cumulus_relay_chain_inprocess_interface::RelayChainInProcessInterface\">RelayChainInProcessInterface</a>&lt;Client&gt;<span class=\"where fmt-newline\">where\n    Client: ProvideRuntimeApi&lt;PBlock&gt; + BlockchainEvents&lt;PBlock&gt; + AuxStore + UsageProvider&lt;PBlock&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.69.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,\n    Client::Api: ParachainHost&lt;PBlock&gt; + BabeApi&lt;PBlock&gt;,</span>"]],
"cumulus_relay_chain_interface":[],
"cumulus_relay_chain_rpc_interface":[["impl RelayChainInterface for <a class=\"struct\" href=\"cumulus_relay_chain_rpc_interface/struct.RelayChainRpcInterface.html\" title=\"struct cumulus_relay_chain_rpc_interface::RelayChainRpcInterface\">RelayChainRpcInterface</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()