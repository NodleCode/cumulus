(function() {var implementors = {
"asset_hub_westend_runtime":[["impl&lt;T: Config&lt;MultiAssetId = <a class=\"struct\" href=\"https://doc.rust-lang.org/1.71.0/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;MultiLocation&gt;, AssetId = MultiLocation&gt;&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"asset_hub_westend_runtime/migrations/struct.NativeAssetParents0ToParents1Migration.html\" title=\"struct asset_hub_westend_runtime::migrations::NativeAssetParents0ToParents1Migration\">NativeAssetParents0ToParents1Migration</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    &lt;T as Config&gt;::PoolAssetId: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.71.0/std/primitive.u32.html\">u32</a>&gt;,\n    AccountIdOf&lt;<a class=\"struct\" href=\"asset_hub_westend_runtime/struct.Runtime.html\" title=\"struct asset_hub_westend_runtime::Runtime\">Runtime</a>&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.71.0/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.71.0/std/primitive.array.html\">32</a>]&gt;,\n    &lt;T as Config&gt;::AccountId: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;&lt;&lt;T as Config&gt;::RuntimeOrigin as OriginTrait&gt;::AccountId&gt;,\n    &lt;&lt;T as Config&gt;::Lookup as StaticLookup&gt;::Source: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&lt;T as Config&gt;::AccountId&gt;,\n    AccountId32: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&lt;T as Config&gt;::AccountId&gt;,</span>"]],
"collectives_polkadot_runtime":[["impl&lt;T: <a class=\"trait\" href=\"collectives_polkadot_runtime/fellowship/pallet_fellowship_origins/trait.Config.html\" title=\"trait collectives_polkadot_runtime::fellowship::pallet_fellowship_origins::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"collectives_polkadot_runtime/fellowship/pallet_fellowship_origins/struct.Pallet.html\" title=\"struct collectives_polkadot_runtime::fellowship::pallet_fellowship_origins::Pallet\">Pallet</a>&lt;T&gt;"]],
"cumulus_pallet_aura_ext":[["impl&lt;T: <a class=\"trait\" href=\"cumulus_pallet_aura_ext/pallet/trait.Config.html\" title=\"trait cumulus_pallet_aura_ext::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"cumulus_pallet_aura_ext/pallet/struct.Pallet.html\" title=\"struct cumulus_pallet_aura_ext::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"cumulus_pallet_dmp_queue":[["impl&lt;T: <a class=\"trait\" href=\"cumulus_pallet_dmp_queue/pallet/trait.Config.html\" title=\"trait cumulus_pallet_dmp_queue::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"cumulus_pallet_dmp_queue/migration/struct.Migration.html\" title=\"struct cumulus_pallet_dmp_queue::migration::Migration\">Migration</a>&lt;T&gt;"],["impl&lt;T: <a class=\"trait\" href=\"cumulus_pallet_dmp_queue/pallet/trait.Config.html\" title=\"trait cumulus_pallet_dmp_queue::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"cumulus_pallet_dmp_queue/pallet/struct.Pallet.html\" title=\"struct cumulus_pallet_dmp_queue::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"cumulus_pallet_parachain_system":[["impl&lt;T: <a class=\"trait\" href=\"cumulus_pallet_parachain_system/pallet/trait.Config.html\" title=\"trait cumulus_pallet_parachain_system::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"cumulus_pallet_parachain_system/migration/struct.Migration.html\" title=\"struct cumulus_pallet_parachain_system::migration::Migration\">Migration</a>&lt;T&gt;"],["impl&lt;T: <a class=\"trait\" href=\"cumulus_pallet_parachain_system/pallet/trait.Config.html\" title=\"trait cumulus_pallet_parachain_system::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"cumulus_pallet_parachain_system/pallet/struct.Pallet.html\" title=\"struct cumulus_pallet_parachain_system::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"cumulus_pallet_solo_to_para":[["impl&lt;T: <a class=\"trait\" href=\"cumulus_pallet_solo_to_para/pallet/trait.Config.html\" title=\"trait cumulus_pallet_solo_to_para::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"cumulus_pallet_solo_to_para/pallet/struct.Pallet.html\" title=\"struct cumulus_pallet_solo_to_para::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"cumulus_pallet_xcm":[["impl&lt;T: <a class=\"trait\" href=\"cumulus_pallet_xcm/pallet/trait.Config.html\" title=\"trait cumulus_pallet_xcm::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"cumulus_pallet_xcm/pallet/struct.Pallet.html\" title=\"struct cumulus_pallet_xcm::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"cumulus_pallet_xcmp_queue":[["impl&lt;T: <a class=\"trait\" href=\"cumulus_pallet_xcmp_queue/pallet/trait.Config.html\" title=\"trait cumulus_pallet_xcmp_queue::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"cumulus_pallet_xcmp_queue/migration/struct.Migration.html\" title=\"struct cumulus_pallet_xcmp_queue::migration::Migration\">Migration</a>&lt;T&gt;"],["impl&lt;T: <a class=\"trait\" href=\"cumulus_pallet_xcmp_queue/pallet/trait.Config.html\" title=\"trait cumulus_pallet_xcmp_queue::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"cumulus_pallet_xcmp_queue/pallet/struct.Pallet.html\" title=\"struct cumulus_pallet_xcmp_queue::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"cumulus_ping":[["impl&lt;T: <a class=\"trait\" href=\"cumulus_ping/pallet/trait.Config.html\" title=\"trait cumulus_ping::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"cumulus_ping/pallet/struct.Pallet.html\" title=\"struct cumulus_ping::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"cumulus_test_runtime":[["impl OnRuntimeUpgrade for <a class=\"struct\" href=\"cumulus_test_runtime/struct.TestOnRuntimeUpgrade.html\" title=\"struct cumulus_test_runtime::TestOnRuntimeUpgrade\">TestOnRuntimeUpgrade</a>"]],
"pallet_bridge_grandpa":[["impl&lt;T: <a class=\"trait\" href=\"pallet_bridge_grandpa/pallet/trait.Config.html\" title=\"trait pallet_bridge_grandpa::pallet::Config\">Config</a>&lt;I&gt;, I: 'static&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"pallet_bridge_grandpa/pallet/struct.Pallet.html\" title=\"struct pallet_bridge_grandpa::pallet::Pallet\">Pallet</a>&lt;T, I&gt;"]],
"pallet_bridge_messages":[["impl&lt;T: <a class=\"trait\" href=\"pallet_bridge_messages/pallet/trait.Config.html\" title=\"trait pallet_bridge_messages::pallet::Config\">Config</a>&lt;I&gt;, I: 'static&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"pallet_bridge_messages/pallet/struct.Pallet.html\" title=\"struct pallet_bridge_messages::pallet::Pallet\">Pallet</a>&lt;T, I&gt;<span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.71.0/std/primitive.u32.html\">u32</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.0/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;BlockNumberFor&lt;T&gt;&gt;,</span>"]],
"pallet_bridge_parachains":[["impl&lt;T: <a class=\"trait\" href=\"pallet_bridge_parachains/pallet/trait.Config.html\" title=\"trait pallet_bridge_parachains::pallet::Config\">Config</a>&lt;I&gt;, I: 'static&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"pallet_bridge_parachains/pallet/struct.Pallet.html\" title=\"struct pallet_bridge_parachains::pallet::Pallet\">Pallet</a>&lt;T, I&gt;"]],
"pallet_bridge_relayers":[["impl&lt;T: <a class=\"trait\" href=\"pallet_bridge_relayers/pallet/trait.Config.html\" title=\"trait pallet_bridge_relayers::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"pallet_bridge_relayers/pallet/struct.Pallet.html\" title=\"struct pallet_bridge_relayers::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"pallet_collator_selection":[["impl&lt;T: <a class=\"trait\" href=\"pallet_collator_selection/pallet/trait.Config.html\" title=\"trait pallet_collator_selection::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"pallet_collator_selection/migration/v1/struct.MigrateToV1.html\" title=\"struct pallet_collator_selection::migration::v1::MigrateToV1\">MigrateToV1</a>&lt;T&gt;"],["impl&lt;T: <a class=\"trait\" href=\"pallet_collator_selection/pallet/trait.Config.html\" title=\"trait pallet_collator_selection::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"pallet_collator_selection/pallet/struct.Pallet.html\" title=\"struct pallet_collator_selection::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"pallet_parachain_template":[["impl&lt;T: <a class=\"trait\" href=\"pallet_parachain_template/pallet/trait.Config.html\" title=\"trait pallet_parachain_template::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"pallet_parachain_template/pallet/struct.Pallet.html\" title=\"struct pallet_parachain_template::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"parachain_info":[["impl&lt;T: <a class=\"trait\" href=\"parachain_info/pallet/trait.Config.html\" title=\"trait parachain_info::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"parachain_info/pallet/struct.Pallet.html\" title=\"struct parachain_info::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"rococo_parachain_runtime":[["impl OnRuntimeUpgrade for <a class=\"struct\" href=\"rococo_parachain_runtime/struct.RemoveCollectiveFlip.html\" title=\"struct rococo_parachain_runtime::RemoveCollectiveFlip\">RemoveCollectiveFlip</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()