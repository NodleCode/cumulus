(function() {var implementors = {
"collectives_polkadot_runtime":[["impl&lt;TreasuryAcc, TempAcc, T, I:&nbsp;'static&gt; OnUnbalanced&lt;&lt;&lt;T as Config&lt;I&gt;&gt;::Currency as Currency&lt;&lt;T as Config&gt;::AccountId&gt;&gt;::NegativeImbalance&gt; for <a class=\"struct\" href=\"collectives_polkadot_runtime/impls/struct.ToParentTreasury.html\" title=\"struct collectives_polkadot_runtime::impls::ToParentTreasury\">ToParentTreasury</a>&lt;TreasuryAcc, TempAcc, T, I&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;TreasuryAcc: Get&lt;&lt;T as Config&gt;::AccountId&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;TempAcc: Get&lt;&lt;T as Config&gt;::AccountId&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Config + Config + Config&lt;I&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.67.1/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.67.1/std/primitive.array.html\">32</a>]: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&lt;T as Config&gt;::AccountId&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;T as Config&lt;I&gt;&gt;::Currency as Currency&lt;&lt;T as Config&gt;::AccountId&gt;&gt;::Balance: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;Fungibility&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;T as Config&gt;::RuntimeOrigin as OriginTrait&gt;::AccountId: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&lt;T as Config&gt;::AccountId&gt;,</span>"]],
"parachains_common":[["impl&lt;R&gt; OnUnbalanced&lt;&lt;Pallet&lt;R, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.67.1/std/primitive.unit.html\">()</a>&gt; as Currency&lt;&lt;R as Config&gt;::AccountId&gt;&gt;::NegativeImbalance&gt; for <a class=\"struct\" href=\"parachains_common/impls/struct.ToStakingPot.html\" title=\"struct parachains_common::impls::ToStakingPot\">ToStakingPot</a>&lt;R&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Config + Config,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"type\" href=\"parachains_common/impls/type.AccountIdOf.html\" title=\"type parachains_common::impls::AccountIdOf\">AccountIdOf</a>&lt;R&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;AccountId&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;AccountId&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;R as Config&gt;::RuntimeEvent: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Event&lt;R&gt;&gt;,</span>"],["impl&lt;R&gt; OnUnbalanced&lt;&lt;Pallet&lt;R, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.67.1/std/primitive.unit.html\">()</a>&gt; as Currency&lt;&lt;R as Config&gt;::AccountId&gt;&gt;::NegativeImbalance&gt; for <a class=\"struct\" href=\"parachains_common/impls/struct.DealWithFees.html\" title=\"struct parachains_common::impls::DealWithFees\">DealWithFees</a>&lt;R&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Config + Config,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"type\" href=\"parachains_common/impls/type.AccountIdOf.html\" title=\"type parachains_common::impls::AccountIdOf\">AccountIdOf</a>&lt;R&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;AccountId&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;AccountId&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;R as Config&gt;::RuntimeEvent: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.67.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Event&lt;R&gt;&gt;,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()