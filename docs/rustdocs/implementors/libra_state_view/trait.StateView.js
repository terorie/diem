(function() {var implementors = {};
implementors["language_e2e_tests"] = [{"text":"impl <a class=\"trait\" href=\"libra_state_view/trait.StateView.html\" title=\"trait libra_state_view::StateView\">StateView</a> for <a class=\"struct\" href=\"language_e2e_tests/data_store/struct.FakeDataStore.html\" title=\"struct language_e2e_tests::data_store::FakeDataStore\">FakeDataStore</a>","synthetic":false,"types":["language_e2e_tests::data_store::FakeDataStore"]}];
implementors["libra_validator_interface"] = [{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"libra_state_view/trait.StateView.html\" title=\"trait libra_state_view::StateView\">StateView</a> for <a class=\"struct\" href=\"libra_validator_interface/struct.DebuggerStateView.html\" title=\"struct libra_validator_interface::DebuggerStateView\">DebuggerStateView</a>&lt;'a&gt;","synthetic":false,"types":["libra_validator_interface::DebuggerStateView"]}];
implementors["libra_vm"] = [{"text":"impl&lt;'block&gt; <a class=\"trait\" href=\"libra_state_view/trait.StateView.html\" title=\"trait libra_state_view::StateView\">StateView</a> for <a class=\"struct\" href=\"libra_vm/data_cache/struct.StateViewCache.html\" title=\"struct libra_vm::data_cache::StateViewCache\">StateViewCache</a>&lt;'block&gt;","synthetic":false,"types":["libra_vm::data_cache::StateViewCache"]}];
implementors["resource_viewer"] = [{"text":"impl <a class=\"trait\" href=\"libra_state_view/trait.StateView.html\" title=\"trait libra_state_view::StateView\">StateView</a> for <a class=\"struct\" href=\"resource_viewer/struct.NullStateView.html\" title=\"struct resource_viewer::NullStateView\">NullStateView</a>","synthetic":false,"types":["resource_viewer::NullStateView"]}];
implementors["storage_interface"] = [{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"libra_state_view/trait.StateView.html\" title=\"trait libra_state_view::StateView\">StateView</a> for <a class=\"struct\" href=\"storage_interface/state_view/struct.VerifiedStateView.html\" title=\"struct storage_interface::state_view::VerifiedStateView\">VerifiedStateView</a>&lt;'a&gt;","synthetic":false,"types":["storage_interface::state_view::VerifiedStateView"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()