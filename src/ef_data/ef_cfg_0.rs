#[doc = "Register `ef_cfg_0` reader"]
pub struct R(crate::R<EF_CFG_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_CFG_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EF_CFG_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EF_CFG_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ef_cfg_0` writer"]
pub struct W(crate::W<EF_CFG_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EF_CFG_0_SPEC>;
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
impl From<crate::W<EF_CFG_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EF_CFG_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ef_sf_aes_mode` reader - "]
pub type EF_SF_AES_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_sf_aes_mode` writer - "]
pub type EF_SF_AES_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EF_CFG_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `ef_ai_dis` reader - "]
pub type EF_AI_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_ai_dis` writer - "]
pub type EF_AI_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_CFG_0_SPEC, bool, O>;
#[doc = "Field `ef_cpu0_dis` reader - "]
pub type EF_CPU0_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_cpu0_dis` writer - "]
pub type EF_CPU0_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_CFG_0_SPEC, bool, O>;
#[doc = "Field `ef_sboot_en` reader - "]
pub type EF_SBOOT_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_sboot_en` writer - "]
pub type EF_SBOOT_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EF_CFG_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `ef_uart_dis` reader - "]
pub type EF_UART_DIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_uart_dis` writer - "]
pub type EF_UART_DIS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EF_CFG_0_SPEC, u8, u8, 4, O>;
#[doc = "Field `ef_ble2_dis` reader - "]
pub type EF_BLE2_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_ble2_dis` writer - "]
pub type EF_BLE2_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_CFG_0_SPEC, bool, O>;
#[doc = "Field `ef_m1542_dis` reader - "]
pub type EF_M1542_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_m1542_dis` writer - "]
pub type EF_M1542_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_CFG_0_SPEC, bool, O>;
#[doc = "Field `ef_sf_key_re_sel` reader - "]
pub type EF_SF_KEY_RE_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_sf_key_re_sel` writer - "]
pub type EF_SF_KEY_RE_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EF_CFG_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `ef_sdu_dis` reader - "]
pub type EF_SDU_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_sdu_dis` writer - "]
pub type EF_SDU_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_CFG_0_SPEC, bool, O>;
#[doc = "Field `ef_btdm_dis` reader - "]
pub type EF_BTDM_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_btdm_dis` writer - "]
pub type EF_BTDM_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_CFG_0_SPEC, bool, O>;
#[doc = "Field `ef_wifi_dis` reader - "]
pub type EF_WIFI_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_wifi_dis` writer - "]
pub type EF_WIFI_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_CFG_0_SPEC, bool, O>;
#[doc = "Field `ef_0_key_enc_en` reader - "]
pub type EF_0_KEY_ENC_EN_R = crate::BitReader<bool>;
#[doc = "Field `ef_0_key_enc_en` writer - "]
pub type EF_0_KEY_ENC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_CFG_0_SPEC, bool, O>;
#[doc = "Field `ef_cam_dis` reader - "]
pub type EF_CAM_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_cam_dis` writer - "]
pub type EF_CAM_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_CFG_0_SPEC, bool, O>;
#[doc = "Field `ef_m154_dis` reader - "]
pub type EF_M154_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_m154_dis` writer - "]
pub type EF_M154_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_CFG_0_SPEC, bool, O>;
#[doc = "Field `ef_cpu1_dis` reader - "]
pub type EF_CPU1_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_cpu1_dis` writer - "]
pub type EF_CPU1_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_CFG_0_SPEC, bool, O>;
#[doc = "Field `ef_cpu_rst_dbg_dis` reader - "]
pub type EF_CPU_RST_DBG_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_cpu_rst_dbg_dis` writer - "]
pub type EF_CPU_RST_DBG_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_CFG_0_SPEC, bool, O>;
#[doc = "Field `ef_se_dbg_dis` reader - "]
pub type EF_SE_DBG_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_se_dbg_dis` writer - "]
pub type EF_SE_DBG_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_CFG_0_SPEC, bool, O>;
#[doc = "Field `ef_efuse_dbg_dis` reader - "]
pub type EF_EFUSE_DBG_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_efuse_dbg_dis` writer - "]
pub type EF_EFUSE_DBG_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_CFG_0_SPEC, bool, O>;
#[doc = "Field `ef_dbg_jtag_1_dis` reader - "]
pub type EF_DBG_JTAG_1_DIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_dbg_jtag_1_dis` writer - "]
pub type EF_DBG_JTAG_1_DIS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EF_CFG_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `ef_dbg_jtag_0_dis` reader - "]
pub type EF_DBG_JTAG_0_DIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_dbg_jtag_0_dis` writer - "]
pub type EF_DBG_JTAG_0_DIS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EF_CFG_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `ef_dbg_mode` reader - "]
pub type EF_DBG_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_dbg_mode` writer - "]
pub type EF_DBG_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EF_CFG_0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ef_sf_aes_mode(&self) -> EF_SF_AES_MODE_R {
        EF_SF_AES_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ef_ai_dis(&self) -> EF_AI_DIS_R {
        EF_AI_DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ef_cpu0_dis(&self) -> EF_CPU0_DIS_R {
        EF_CPU0_DIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn ef_sboot_en(&self) -> EF_SBOOT_EN_R {
        EF_SBOOT_EN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:9"]
    #[inline(always)]
    pub fn ef_uart_dis(&self) -> EF_UART_DIS_R {
        EF_UART_DIS_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ef_ble2_dis(&self) -> EF_BLE2_DIS_R {
        EF_BLE2_DIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ef_m1542_dis(&self) -> EF_M1542_DIS_R {
        EF_M1542_DIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn ef_sf_key_re_sel(&self) -> EF_SF_KEY_RE_SEL_R {
        EF_SF_KEY_RE_SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ef_sdu_dis(&self) -> EF_SDU_DIS_R {
        EF_SDU_DIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ef_btdm_dis(&self) -> EF_BTDM_DIS_R {
        EF_BTDM_DIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ef_wifi_dis(&self) -> EF_WIFI_DIS_R {
        EF_WIFI_DIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ef_0_key_enc_en(&self) -> EF_0_KEY_ENC_EN_R {
        EF_0_KEY_ENC_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ef_cam_dis(&self) -> EF_CAM_DIS_R {
        EF_CAM_DIS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ef_m154_dis(&self) -> EF_M154_DIS_R {
        EF_M154_DIS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ef_cpu1_dis(&self) -> EF_CPU1_DIS_R {
        EF_CPU1_DIS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ef_cpu_rst_dbg_dis(&self) -> EF_CPU_RST_DBG_DIS_R {
        EF_CPU_RST_DBG_DIS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ef_se_dbg_dis(&self) -> EF_SE_DBG_DIS_R {
        EF_SE_DBG_DIS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ef_efuse_dbg_dis(&self) -> EF_EFUSE_DBG_DIS_R {
        EF_EFUSE_DBG_DIS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ef_dbg_jtag_1_dis(&self) -> EF_DBG_JTAG_1_DIS_R {
        EF_DBG_JTAG_1_DIS_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn ef_dbg_jtag_0_dis(&self) -> EF_DBG_JTAG_0_DIS_R {
        EF_DBG_JTAG_0_DIS_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn ef_dbg_mode(&self) -> EF_DBG_MODE_R {
        EF_DBG_MODE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn ef_sf_aes_mode(&mut self) -> EF_SF_AES_MODE_W<0> {
        EF_SF_AES_MODE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ef_ai_dis(&mut self) -> EF_AI_DIS_W<2> {
        EF_AI_DIS_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ef_cpu0_dis(&mut self) -> EF_CPU0_DIS_W<3> {
        EF_CPU0_DIS_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn ef_sboot_en(&mut self) -> EF_SBOOT_EN_W<4> {
        EF_SBOOT_EN_W::new(self)
    }
    #[doc = "Bits 6:9"]
    #[inline(always)]
    #[must_use]
    pub fn ef_uart_dis(&mut self) -> EF_UART_DIS_W<6> {
        EF_UART_DIS_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn ef_ble2_dis(&mut self) -> EF_BLE2_DIS_W<10> {
        EF_BLE2_DIS_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn ef_m1542_dis(&mut self) -> EF_M1542_DIS_W<11> {
        EF_M1542_DIS_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn ef_sf_key_re_sel(&mut self) -> EF_SF_KEY_RE_SEL_W<12> {
        EF_SF_KEY_RE_SEL_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn ef_sdu_dis(&mut self) -> EF_SDU_DIS_W<14> {
        EF_SDU_DIS_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn ef_btdm_dis(&mut self) -> EF_BTDM_DIS_W<15> {
        EF_BTDM_DIS_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn ef_wifi_dis(&mut self) -> EF_WIFI_DIS_W<16> {
        EF_WIFI_DIS_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn ef_0_key_enc_en(&mut self) -> EF_0_KEY_ENC_EN_W<17> {
        EF_0_KEY_ENC_EN_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn ef_cam_dis(&mut self) -> EF_CAM_DIS_W<18> {
        EF_CAM_DIS_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn ef_m154_dis(&mut self) -> EF_M154_DIS_W<19> {
        EF_M154_DIS_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn ef_cpu1_dis(&mut self) -> EF_CPU1_DIS_W<20> {
        EF_CPU1_DIS_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn ef_cpu_rst_dbg_dis(&mut self) -> EF_CPU_RST_DBG_DIS_W<21> {
        EF_CPU_RST_DBG_DIS_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn ef_se_dbg_dis(&mut self) -> EF_SE_DBG_DIS_W<22> {
        EF_SE_DBG_DIS_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn ef_efuse_dbg_dis(&mut self) -> EF_EFUSE_DBG_DIS_W<23> {
        EF_EFUSE_DBG_DIS_W::new(self)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn ef_dbg_jtag_1_dis(&mut self) -> EF_DBG_JTAG_1_DIS_W<24> {
        EF_DBG_JTAG_1_DIS_W::new(self)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    #[must_use]
    pub fn ef_dbg_jtag_0_dis(&mut self) -> EF_DBG_JTAG_0_DIS_W<26> {
        EF_DBG_JTAG_0_DIS_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn ef_dbg_mode(&mut self) -> EF_DBG_MODE_W<28> {
        EF_DBG_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ef_cfg_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_cfg_0](index.html) module"]
pub struct EF_CFG_0_SPEC;
impl crate::RegisterSpec for EF_CFG_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_cfg_0::R](R) reader structure"]
impl crate::Readable for EF_CFG_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ef_cfg_0::W](W) writer structure"]
impl crate::Writable for EF_CFG_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ef_cfg_0 to value 0"]
impl crate::Resettable for EF_CFG_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}