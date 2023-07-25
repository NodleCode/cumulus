(function() {var implementors = {
"cumulus_client_consensus_aura":[["impl&lt;B, CIDP, W&gt; ParachainConsensus&lt;B&gt; for <a class=\"struct\" href=\"cumulus_client_consensus_aura/struct.AuraConsensus.html\" title=\"struct cumulus_client_consensus_aura::AuraConsensus\">AuraConsensus</a>&lt;B, CIDP, W&gt;<span class=\"where fmt-newline\">where\n    B: BlockT,\n    CIDP: CreateInherentDataProviders&lt;B, (PHash, PersistedValidationData)&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,\n    CIDP::InherentDataProviders: <a class=\"trait\" href=\"cumulus_client_consensus_aura/trait.InherentDataProviderExt.html\" title=\"trait cumulus_client_consensus_aura::InherentDataProviderExt\">InherentDataProviderExt</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,\n    W: SimpleSlotWorker&lt;B&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,\n    W::Proposer: Proposer&lt;B, Proof = &lt;EnableProofRecording as ProofRecording&gt;::Proof&gt;,</span>"]],
"cumulus_client_consensus_common":[],
"cumulus_client_consensus_relay_chain":[["impl&lt;B, PF, BI, RCInterface, CIDP&gt; ParachainConsensus&lt;B&gt; for <a class=\"struct\" href=\"cumulus_client_consensus_relay_chain/struct.RelayChainConsensus.html\" title=\"struct cumulus_client_consensus_relay_chain::RelayChainConsensus\">RelayChainConsensus</a>&lt;B, PF, BI, RCInterface, CIDP&gt;<span class=\"where fmt-newline\">where\n    B: BlockT,\n    RCInterface: RelayChainInterface + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    BI: BlockImport&lt;B&gt; + ParachainBlockImportMarker + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,\n    PF: Environment&lt;B&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,\n    PF::Proposer: Proposer&lt;B, Transaction = BI::Transaction, ProofRecording = EnableProofRecording, Proof = &lt;EnableProofRecording as ProofRecording&gt;::Proof&gt;,\n    CIDP: CreateInherentDataProviders&lt;B, (PHash, PersistedValidationData)&gt;,</span>"]],
"polkadot_parachain":[["impl&lt;Client, AuraId&gt; ParachainConsensus&lt;Block&lt;Header&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.71.0/std/primitive.u32.html\">u32</a>, BlakeTwo256&gt;, OpaqueExtrinsic&gt;&gt; for <a class=\"struct\" href=\"polkadot_parachain/service/struct.WaitForAuraConsensus.html\" title=\"struct polkadot_parachain::service::WaitForAuraConsensus\">WaitForAuraConsensus</a>&lt;Client, AuraId&gt;<span class=\"where fmt-newline\">where\n    Client: ProvideRuntimeApi&lt;<a class=\"type\" href=\"polkadot_parachain/service/type.Block.html\" title=\"type polkadot_parachain::service::Block\">Block</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,\n    Client::Api: AuraApi&lt;<a class=\"type\" href=\"polkadot_parachain/service/type.Block.html\" title=\"type polkadot_parachain::service::Block\">Block</a>, AuraId&gt;,\n    AuraId: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + Codec + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()