(function() {var implementors = {
"assets_common":[["impl&lt;SelfParaId: Get&lt;ParaId&gt;&gt; ContainsPair&lt;MultiLocation, MultiLocation&gt; for <a class=\"struct\" href=\"assets_common/matching/struct.FromSiblingParachain.html\" title=\"struct assets_common::matching::FromSiblingParachain\">FromSiblingParachain</a>&lt;SelfParaId&gt;"],["impl&lt;Assets, LocalAssetIdConverter, ForeignAssets&gt; ContainsPair&lt;MultiLocation, &lt;&lt;MultiSignature as Verify&gt;::Signer as IdentifyAccount&gt;::AccountId&gt; for <a class=\"struct\" href=\"assets_common/local_and_foreign_assets/struct.LocalAndForeignAssets.html\" title=\"struct assets_common::local_and_foreign_assets::LocalAndForeignAssets\">LocalAndForeignAssets</a>&lt;Assets, LocalAssetIdConverter, ForeignAssets&gt;<span class=\"where fmt-newline\">where\n    Assets: PalletInfoAccess + ContainsPair&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.71.0/std/primitive.u32.html\">u32</a>, AccountId&gt;,\n    LocalAssetIdConverter: MaybeEquivalence&lt;MultiLocation, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.71.0/std/primitive.u32.html\">u32</a>&gt;,\n    ForeignAssets: ContainsPair&lt;MultiLocation, AccountId&gt;,</span>"],["impl&lt;IsForeign: ContainsPair&lt;MultiLocation, MultiLocation&gt;&gt; ContainsPair&lt;MultiAsset, MultiLocation&gt; for <a class=\"struct\" href=\"assets_common/matching/struct.IsForeignConcreteAsset.html\" title=\"struct assets_common::matching::IsForeignConcreteAsset\">IsForeignConcreteAsset</a>&lt;IsForeign&gt;"]],
"parachains_common":[["impl&lt;Location: Get&lt;MultiLocation&gt;&gt; ContainsPair&lt;MultiAsset, MultiLocation&gt; for <a class=\"struct\" href=\"parachains_common/xcm_config/struct.ConcreteNativeAssetFrom.html\" title=\"struct parachains_common::xcm_config::ConcreteNativeAssetFrom\">ConcreteNativeAssetFrom</a>&lt;Location&gt;"],["impl&lt;T: Get&lt;MultiLocation&gt;&gt; ContainsPair&lt;MultiAsset, MultiLocation&gt; for <a class=\"struct\" href=\"parachains_common/impls/struct.AssetsFrom.html\" title=\"struct parachains_common::impls::AssetsFrom\">AssetsFrom</a>&lt;T&gt;"]],
"penpal_runtime":[["impl&lt;T: Get&lt;MultiLocation&gt;&gt; ContainsPair&lt;MultiAsset, MultiLocation&gt; for <a class=\"struct\" href=\"penpal_runtime/xcm_config/struct.AssetsFrom.html\" title=\"struct penpal_runtime::xcm_config::AssetsFrom\">AssetsFrom</a>&lt;T&gt;"],["impl ContainsPair&lt;MultiAsset, MultiLocation&gt; for <a class=\"struct\" href=\"penpal_runtime/xcm_config/struct.MultiNativeAsset.html\" title=\"struct penpal_runtime::xcm_config::MultiNativeAsset\">MultiNativeAsset</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()