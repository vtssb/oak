(function() {var implementors = {
"oak_dice":[["impl FromBytes for <a class=\"struct\" href=\"oak_dice/evidence/struct.Evidence.html\" title=\"struct oak_dice::evidence::Evidence\">Evidence</a><span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"oak_dice/evidence/struct.RootLayerEvidence.html\" title=\"struct oak_dice::evidence::RootLayerEvidence\">RootLayerEvidence</a>: FromBytes,\n    <a class=\"struct\" href=\"oak_dice/evidence/struct.LayerEvidence.html\" title=\"struct oak_dice::evidence::LayerEvidence\">LayerEvidence</a>: FromBytes,\n    <a class=\"struct\" href=\"oak_dice/evidence/struct.ApplicationKeys.html\" title=\"struct oak_dice::evidence::ApplicationKeys\">ApplicationKeys</a>: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_dice/evidence/struct.CertificateAuthority.html\" title=\"struct oak_dice::evidence::CertificateAuthority\">CertificateAuthority</a><span class=\"where fmt-newline\">where\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">64</a>]: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_dice/evidence/struct.Stage0DiceData.html\" title=\"struct oak_dice::evidence::Stage0DiceData\">Stage0DiceData</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>: FromBytes,\n    <a class=\"struct\" href=\"oak_dice/evidence/struct.RootLayerEvidence.html\" title=\"struct oak_dice::evidence::RootLayerEvidence\">RootLayerEvidence</a>: FromBytes,\n    <a class=\"struct\" href=\"oak_dice/evidence/struct.LayerEvidence.html\" title=\"struct oak_dice::evidence::LayerEvidence\">LayerEvidence</a>: FromBytes,\n    <a class=\"struct\" href=\"oak_dice/evidence/struct.CertificateAuthority.html\" title=\"struct oak_dice::evidence::CertificateAuthority\">CertificateAuthority</a>: FromBytes,\n    <a class=\"struct\" href=\"oak_dice/evidence/struct.CompoundDeviceIdentifier.html\" title=\"struct oak_dice::evidence::CompoundDeviceIdentifier\">CompoundDeviceIdentifier</a>: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">640</a>]: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_dice/evidence/struct.RestrictedKernelDiceData.html\" title=\"struct oak_dice::evidence::RestrictedKernelDiceData\">RestrictedKernelDiceData</a><span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"oak_dice/evidence/struct.Evidence.html\" title=\"struct oak_dice::evidence::Evidence\">Evidence</a>: FromBytes,\n    <a class=\"struct\" href=\"oak_dice/evidence/struct.ApplicationPrivateKeys.html\" title=\"struct oak_dice::evidence::ApplicationPrivateKeys\">ApplicationPrivateKeys</a>: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_dice/evidence/struct.LayerEvidence.html\" title=\"struct oak_dice::evidence::LayerEvidence\">LayerEvidence</a><span class=\"where fmt-newline\">where\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">1024</a>]: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_dice/evidence/struct.ApplicationKeys.html\" title=\"struct oak_dice::evidence::ApplicationKeys\">ApplicationKeys</a><span class=\"where fmt-newline\">where\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">1024</a>]: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_dice/evidence/struct.RootLayerEvidence.html\" title=\"struct oak_dice::evidence::RootLayerEvidence\">RootLayerEvidence</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">2048</a>]: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">256</a>]: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_dice/evidence/struct.ApplicationPrivateKeys.html\" title=\"struct oak_dice::evidence::ApplicationPrivateKeys\">ApplicationPrivateKeys</a><span class=\"where fmt-newline\">where\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">64</a>]: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_dice/evidence/struct.CompoundDeviceIdentifier.html\" title=\"struct oak_dice::evidence::CompoundDeviceIdentifier\">CompoundDeviceIdentifier</a><span class=\"where fmt-newline\">where\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">32</a>]: FromBytes,</span>"]],
"oak_linux_boot_params":[["impl FromBytes for <a class=\"struct\" href=\"oak_linux_boot_params/struct.BootE820Entry.html\" title=\"struct oak_linux_boot_params::BootE820Entry\">BootE820Entry</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>: FromBytes,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_linux_boot_params/struct.SetupHeader.html\" title=\"struct oak_linux_boot_params::SetupHeader\">SetupHeader</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>: FromBytes,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a>: FromBytes,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>: FromBytes,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>: FromBytes,</span>"]],
"oak_sev_guest":[["impl FromBytes for <a class=\"struct\" href=\"oak_sev_guest/guest/struct.EcdsaSignature.html\" title=\"struct oak_sev_guest::guest::EcdsaSignature\">EcdsaSignature</a><span class=\"where fmt-newline\">where\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">72</a>]: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">368</a>]: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_sev_guest/guest/struct.AttestationResponse.html\" title=\"struct oak_sev_guest::guest::AttestationResponse\">AttestationResponse</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">24</a>]: FromBytes,\n    <a class=\"struct\" href=\"oak_sev_guest/guest/struct.AttestationReport.html\" title=\"struct oak_sev_guest::guest::AttestationReport\">AttestationReport</a>: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_sev_guest/guest/struct.AuthenticatedHeader.html\" title=\"struct oak_sev_guest::guest::AuthenticatedHeader\">AuthenticatedHeader</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>: FromBytes,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a>: FromBytes,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">35</a>]: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_sev_guest/guest/struct.KeyRequest.html\" title=\"struct oak_sev_guest::guest::KeyRequest\">KeyRequest</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>: FromBytes,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_sev_guest/ghcb/struct.ValidBitmap.html\" title=\"struct oak_sev_guest::ghcb::ValidBitmap\">ValidBitmap</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u128.html\">u128</a>: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_sev_guest/guest/struct.GuestPolicy.html\" title=\"struct oak_sev_guest::guest::GuestPolicy\">GuestPolicy</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>: FromBytes,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a>: FromBytes,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_sev_guest/guest/struct.GuestMessageHeader.html\" title=\"struct oak_sev_guest::guest::GuestMessageHeader\">GuestMessageHeader</a><span class=\"where fmt-newline\">where\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">32</a>]: FromBytes,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>: FromBytes,\n    <a class=\"struct\" href=\"oak_sev_guest/guest/struct.AuthenticatedHeader.html\" title=\"struct oak_sev_guest::guest::AuthenticatedHeader\">AuthenticatedHeader</a>: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_sev_guest/vmsa/struct.Vmsa.html\" title=\"struct oak_sev_guest::vmsa::Vmsa\">Vmsa</a><span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"oak_sev_guest/vmsa/struct.SegmentRegister.html\" title=\"struct oak_sev_guest::vmsa::SegmentRegister\">SegmentRegister</a>: FromBytes,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>: FromBytes,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a>: FromBytes,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>: FromBytes,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">104</a>]: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">24</a>]: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">32</a>]: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">80</a>]: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">16</a>]: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">256</a>]: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_sev_guest/guest/struct.AttestationReport.html\" title=\"struct oak_sev_guest::guest::AttestationReport\">AttestationReport</a><span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"oak_sev_guest/guest/struct.AttestationReportData.html\" title=\"struct oak_sev_guest::guest::AttestationReportData\">AttestationReportData</a>: FromBytes,\n    <a class=\"struct\" href=\"oak_sev_guest/guest/struct.EcdsaSignature.html\" title=\"struct oak_sev_guest::guest::EcdsaSignature\">EcdsaSignature</a>: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_sev_guest/guest/struct.GuestMessage.html\" title=\"struct oak_sev_guest::guest::GuestMessage\">GuestMessage</a><span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"oak_sev_guest/guest/struct.GuestMessageHeader.html\" title=\"struct oak_sev_guest::guest::GuestMessageHeader\">GuestMessageHeader</a>: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">4000</a>]: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_sev_guest/guest/struct.AttestationRequest.html\" title=\"struct oak_sev_guest::guest::AttestationRequest\">AttestationRequest</a><span class=\"where fmt-newline\">where\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">64</a>]: FromBytes,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">28</a>]: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_sev_guest/cpuid/struct.CpuidFunction.html\" title=\"struct oak_sev_guest::cpuid::CpuidFunction\">CpuidFunction</a><span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"oak_sev_guest/cpuid/struct.CpuidInput.html\" title=\"struct oak_sev_guest::cpuid::CpuidInput\">CpuidInput</a>: FromBytes,\n    <a class=\"struct\" href=\"oak_sev_guest/cpuid/struct.CpuidOutput.html\" title=\"struct oak_sev_guest::cpuid::CpuidOutput\">CpuidOutput</a>: FromBytes,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_sev_guest/guest/struct.AttestationReportData.html\" title=\"struct oak_sev_guest::guest::AttestationReportData\">AttestationReportData</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>: FromBytes,\n    <a class=\"struct\" href=\"oak_sev_guest/guest/struct.GuestPolicy.html\" title=\"struct oak_sev_guest::guest::GuestPolicy\">GuestPolicy</a>: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">16</a>]: FromBytes,\n    <a class=\"struct\" href=\"oak_sev_guest/guest/struct.TcbVersion.html\" title=\"struct oak_sev_guest::guest::TcbVersion\">TcbVersion</a>: FromBytes,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">64</a>]: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">48</a>]: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">32</a>]: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">24</a>]: FromBytes,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">168</a>]: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_sev_guest/secrets/struct.SecretsPage.html\" title=\"struct oak_sev_guest::secrets::SecretsPage\">SecretsPage</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">16</a>]: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">32</a>]: FromBytes,\n    <a class=\"struct\" href=\"oak_sev_guest/secrets/struct.GuestReservedArea.html\" title=\"struct oak_sev_guest::secrets::GuestReservedArea\">GuestReservedArea</a>: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">64</a>]: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_sev_guest/secrets/struct.GuestReservedArea.html\" title=\"struct oak_sev_guest::secrets::GuestReservedArea\">GuestReservedArea</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>: FromBytes,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">22</a>]: FromBytes,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a>: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">32</a>]: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_sev_guest/guest/struct.TcbVersion.html\" title=\"struct oak_sev_guest::guest::TcbVersion\">TcbVersion</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">4</a>]: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_sev_guest/cpuid/struct.CpuidPage.html\" title=\"struct oak_sev_guest::cpuid::CpuidPage\">CpuidPage</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">12</a>]: FromBytes,\n    [<a class=\"struct\" href=\"oak_sev_guest/cpuid/struct.CpuidFunction.html\" title=\"struct oak_sev_guest::cpuid::CpuidFunction\">CpuidFunction</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">64</a>]: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_sev_guest/vmsa/struct.SegmentRegister.html\" title=\"struct oak_sev_guest::vmsa::SegmentRegister\">SegmentRegister</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a>: FromBytes,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>: FromBytes,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_sev_guest/guest/struct.KeyResponse.html\" title=\"struct oak_sev_guest::guest::KeyResponse\">KeyResponse</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">28</a>]: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">32</a>]: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_sev_guest/cpuid/struct.CpuidOutput.html\" title=\"struct oak_sev_guest::cpuid::CpuidOutput\">CpuidOutput</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_sev_guest/cpuid/struct.CpuidInput.html\" title=\"struct oak_sev_guest::cpuid::CpuidInput\">CpuidInput</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>: FromBytes,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_sev_guest/vmsa/struct.VmsaPage.html\" title=\"struct oak_sev_guest::vmsa::VmsaPage\">VmsaPage</a><span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"oak_sev_guest/vmsa/struct.Vmsa.html\" title=\"struct oak_sev_guest::vmsa::Vmsa\">Vmsa</a>: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">2104</a>]: FromBytes,</span>"],["impl FromBytes for <a class=\"struct\" href=\"oak_sev_guest/ghcb/struct.Ghcb.html\" title=\"struct oak_sev_guest::ghcb::Ghcb\">Ghcb</a><span class=\"where fmt-newline\">where\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">203</a>]: FromBytes,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">116</a>]: FromBytes,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">24</a>]: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">144</a>]: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">264</a>]: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">112</a>]: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">56</a>]: FromBytes,\n    <a class=\"struct\" href=\"oak_sev_guest/ghcb/struct.ValidBitmap.html\" title=\"struct oak_sev_guest::ghcb::ValidBitmap\">ValidBitmap</a>: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">1016</a>]: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">2032</a>]: FromBytes,\n    [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">10</a>]: FromBytes,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a>: FromBytes,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>: FromBytes,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()