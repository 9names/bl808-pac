#[doc = "Register `cgen_cfg2` reader"]
pub struct R(crate::R<CGEN_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CGEN_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CGEN_CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CGEN_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cgen_cfg2` writer"]
pub struct W(crate::W<CGEN_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CGEN_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CGEN_CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CGEN_CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cgen_s0` reader - "]
pub type CGEN_S0_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s0` writer - "]
pub type CGEN_S0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG2_SPEC, bool, O>;
#[doc = "Field `reserved_1_3` reader - "]
pub type RESERVED_1_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cgen_s2_wifi` reader - "]
pub type CGEN_S2_WIFI_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s2_wifi` writer - "]
pub type CGEN_S2_WIFI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG2_SPEC, bool, O>;
#[doc = "Field `reserved_5_9` reader - "]
pub type RESERVED_5_9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cgen_s3_bt_ble2` reader - "]
pub type CGEN_S3_BT_BLE2_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s3_bt_ble2` writer - "]
pub type CGEN_S3_BT_BLE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG2_SPEC, bool, O>;
#[doc = "Field `cgen_s3_m1542` reader - "]
pub type CGEN_S3_M1542_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s3_m1542` writer - "]
pub type CGEN_S3_M1542_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG2_SPEC, bool, O>;
#[doc = "Field `reserved_12_15` reader - "]
pub type RESERVED_12_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cgen_s1_ext_emi_misc` reader - "]
pub type CGEN_S1_EXT_EMI_MISC_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1_ext_emi_misc` writer - "]
pub type CGEN_S1_EXT_EMI_MISC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CGEN_CFG2_SPEC, bool, O>;
#[doc = "Field `cgen_s1_ext_psram0_ctrl` reader - "]
pub type CGEN_S1_EXT_PSRAM0_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1_ext_psram0_ctrl` writer - "]
pub type CGEN_S1_EXT_PSRAM0_CTRL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CGEN_CFG2_SPEC, bool, O>;
#[doc = "Field `cgen_s1_ext_psram_ctrl` reader - "]
pub type CGEN_S1_EXT_PSRAM_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1_ext_psram_ctrl` writer - "]
pub type CGEN_S1_EXT_PSRAM_CTRL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CGEN_CFG2_SPEC, bool, O>;
#[doc = "Field `cgen_s1_ext_usb` reader - "]
pub type CGEN_S1_EXT_USB_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1_ext_usb` writer - "]
pub type CGEN_S1_EXT_USB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG2_SPEC, bool, O>;
#[doc = "Field `cgen_s1_ext_mix2` reader - "]
pub type CGEN_S1_EXT_MIX2_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1_ext_mix2` writer - "]
pub type CGEN_S1_EXT_MIX2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG2_SPEC, bool, O>;
#[doc = "Field `cgen_s1_ext_audio` reader - "]
pub type CGEN_S1_EXT_AUDIO_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1_ext_audio` writer - "]
pub type CGEN_S1_EXT_AUDIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG2_SPEC, bool, O>;
#[doc = "Field `cgen_s1_ext_sdh` reader - "]
pub type CGEN_S1_EXT_SDH_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1_ext_sdh` writer - "]
pub type CGEN_S1_EXT_SDH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG2_SPEC, bool, O>;
#[doc = "Field `cgen_s1_ext_emac` reader - "]
pub type CGEN_S1_EXT_EMAC_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1_ext_emac` writer - "]
pub type CGEN_S1_EXT_EMAC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG2_SPEC, bool, O>;
#[doc = "Field `cgen_s1_ext_dma2` reader - "]
pub type CGEN_S1_EXT_DMA2_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1_ext_dma2` writer - "]
pub type CGEN_S1_EXT_DMA2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG2_SPEC, bool, O>;
#[doc = "Field `cgen_s1_ext_rsvd9` reader - "]
pub type CGEN_S1_EXT_RSVD9_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1_ext_rsvd9` writer - "]
pub type CGEN_S1_EXT_RSVD9_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG2_SPEC, bool, O>;
#[doc = "Field `cgen_s1_ext_rsvd10` reader - "]
pub type CGEN_S1_EXT_RSVD10_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1_ext_rsvd10` writer - "]
pub type CGEN_S1_EXT_RSVD10_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG2_SPEC, bool, O>;
#[doc = "Field `cgen_s1_ext_rsvd11` reader - "]
pub type CGEN_S1_EXT_RSVD11_R = crate::BitReader<bool>;
#[doc = "Field `cgen_s1_ext_rsvd11` writer - "]
pub type CGEN_S1_EXT_RSVD11_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG2_SPEC, bool, O>;
#[doc = "Field `reserved_28_31` reader - "]
pub type RESERVED_28_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cgen_s0(&self) -> CGEN_S0_R {
        CGEN_S0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn reserved_1_3(&self) -> RESERVED_1_3_R {
        RESERVED_1_3_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cgen_s2_wifi(&self) -> CGEN_S2_WIFI_R {
        CGEN_S2_WIFI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn reserved_5_9(&self) -> RESERVED_5_9_R {
        RESERVED_5_9_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cgen_s3_bt_ble2(&self) -> CGEN_S3_BT_BLE2_R {
        CGEN_S3_BT_BLE2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cgen_s3_m1542(&self) -> CGEN_S3_M1542_R {
        CGEN_S3_M1542_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn reserved_12_15(&self) -> RESERVED_12_15_R {
        RESERVED_12_15_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cgen_s1_ext_emi_misc(&self) -> CGEN_S1_EXT_EMI_MISC_R {
        CGEN_S1_EXT_EMI_MISC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cgen_s1_ext_psram0_ctrl(&self) -> CGEN_S1_EXT_PSRAM0_CTRL_R {
        CGEN_S1_EXT_PSRAM0_CTRL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cgen_s1_ext_psram_ctrl(&self) -> CGEN_S1_EXT_PSRAM_CTRL_R {
        CGEN_S1_EXT_PSRAM_CTRL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cgen_s1_ext_usb(&self) -> CGEN_S1_EXT_USB_R {
        CGEN_S1_EXT_USB_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cgen_s1_ext_mix2(&self) -> CGEN_S1_EXT_MIX2_R {
        CGEN_S1_EXT_MIX2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cgen_s1_ext_audio(&self) -> CGEN_S1_EXT_AUDIO_R {
        CGEN_S1_EXT_AUDIO_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn cgen_s1_ext_sdh(&self) -> CGEN_S1_EXT_SDH_R {
        CGEN_S1_EXT_SDH_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn cgen_s1_ext_emac(&self) -> CGEN_S1_EXT_EMAC_R {
        CGEN_S1_EXT_EMAC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cgen_s1_ext_dma2(&self) -> CGEN_S1_EXT_DMA2_R {
        CGEN_S1_EXT_DMA2_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cgen_s1_ext_rsvd9(&self) -> CGEN_S1_EXT_RSVD9_R {
        CGEN_S1_EXT_RSVD9_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cgen_s1_ext_rsvd10(&self) -> CGEN_S1_EXT_RSVD10_R {
        CGEN_S1_EXT_RSVD10_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cgen_s1_ext_rsvd11(&self) -> CGEN_S1_EXT_RSVD11_R {
        CGEN_S1_EXT_RSVD11_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn reserved_28_31(&self) -> RESERVED_28_31_R {
        RESERVED_28_31_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s0(&mut self) -> CGEN_S0_W<0> {
        CGEN_S0_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s2_wifi(&mut self) -> CGEN_S2_WIFI_W<4> {
        CGEN_S2_WIFI_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s3_bt_ble2(&mut self) -> CGEN_S3_BT_BLE2_W<10> {
        CGEN_S3_BT_BLE2_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s3_m1542(&mut self) -> CGEN_S3_M1542_W<11> {
        CGEN_S3_M1542_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_ext_emi_misc(&mut self) -> CGEN_S1_EXT_EMI_MISC_W<16> {
        CGEN_S1_EXT_EMI_MISC_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_ext_psram0_ctrl(&mut self) -> CGEN_S1_EXT_PSRAM0_CTRL_W<17> {
        CGEN_S1_EXT_PSRAM0_CTRL_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_ext_psram_ctrl(&mut self) -> CGEN_S1_EXT_PSRAM_CTRL_W<18> {
        CGEN_S1_EXT_PSRAM_CTRL_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_ext_usb(&mut self) -> CGEN_S1_EXT_USB_W<19> {
        CGEN_S1_EXT_USB_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_ext_mix2(&mut self) -> CGEN_S1_EXT_MIX2_W<20> {
        CGEN_S1_EXT_MIX2_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_ext_audio(&mut self) -> CGEN_S1_EXT_AUDIO_W<21> {
        CGEN_S1_EXT_AUDIO_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_ext_sdh(&mut self) -> CGEN_S1_EXT_SDH_W<22> {
        CGEN_S1_EXT_SDH_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_ext_emac(&mut self) -> CGEN_S1_EXT_EMAC_W<23> {
        CGEN_S1_EXT_EMAC_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_ext_dma2(&mut self) -> CGEN_S1_EXT_DMA2_W<24> {
        CGEN_S1_EXT_DMA2_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_ext_rsvd9(&mut self) -> CGEN_S1_EXT_RSVD9_W<25> {
        CGEN_S1_EXT_RSVD9_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_ext_rsvd10(&mut self) -> CGEN_S1_EXT_RSVD10_W<26> {
        CGEN_S1_EXT_RSVD10_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_s1_ext_rsvd11(&mut self) -> CGEN_S1_EXT_RSVD11_W<27> {
        CGEN_S1_EXT_RSVD11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cgen_s1_ext + cgen_s3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgen_cfg2](index.html) module"]
pub struct CGEN_CFG2_SPEC;
impl crate::RegisterSpec for CGEN_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cgen_cfg2::R](R) reader structure"]
impl crate::Readable for CGEN_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cgen_cfg2::W](W) writer structure"]
impl crate::Writable for CGEN_CFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cgen_cfg2 to value 0x0fff_0c11"]
impl crate::Resettable for CGEN_CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff_0c11;
}
