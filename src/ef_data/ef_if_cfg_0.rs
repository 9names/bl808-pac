#[doc = "Register `ef_if_cfg_0` reader"]
pub struct R(crate::R<EF_IF_CFG_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_IF_CFG_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EF_IF_CFG_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EF_IF_CFG_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ef_if_cfg_0` writer"]
pub struct W(crate::W<EF_IF_CFG_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EF_IF_CFG_0_SPEC>;
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
impl From<crate::W<EF_IF_CFG_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EF_IF_CFG_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ef_if_sf_aes_mode` reader - "]
pub type EF_IF_SF_AES_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_if_ai_dis` reader - "]
pub type EF_IF_AI_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_cpu0_dis` reader - "]
pub type EF_IF_CPU0_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_sboot_en` reader - "]
pub type EF_IF_SBOOT_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_if_uart_dis` reader - "]
pub type EF_IF_UART_DIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_if_ble2_dis` reader - "]
pub type EF_IF_BLE2_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_m1542_dis` reader - "]
pub type EF_IF_M1542_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_sf_key_re_sel` reader - "]
pub type EF_IF_SF_KEY_RE_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_if_sdu_dis` reader - "]
pub type EF_IF_SDU_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_btdm_dis` reader - "]
pub type EF_IF_BTDM_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_wifi_dis` reader - "]
pub type EF_IF_WIFI_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_0_key_enc_en` reader - "]
pub type EF_IF_0_KEY_ENC_EN_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_cam_dis` reader - "]
pub type EF_IF_CAM_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_m154_dis` reader - "]
pub type EF_IF_M154_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_cpu1_dis` reader - "]
pub type EF_IF_CPU1_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_cpu_rst_dbg_dis` reader - "]
pub type EF_IF_CPU_RST_DBG_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_se_dbg_dis` reader - "]
pub type EF_IF_SE_DBG_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_efuse_dbg_dis` reader - "]
pub type EF_IF_EFUSE_DBG_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_dbg_jtag_1_dis` reader - "]
pub type EF_IF_DBG_JTAG_1_DIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_if_dbg_jtag_0_dis` reader - "]
pub type EF_IF_DBG_JTAG_0_DIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_if_dbg_mode` reader - "]
pub type EF_IF_DBG_MODE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ef_if_sf_aes_mode(&self) -> EF_IF_SF_AES_MODE_R {
        EF_IF_SF_AES_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ef_if_ai_dis(&self) -> EF_IF_AI_DIS_R {
        EF_IF_AI_DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ef_if_cpu0_dis(&self) -> EF_IF_CPU0_DIS_R {
        EF_IF_CPU0_DIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn ef_if_sboot_en(&self) -> EF_IF_SBOOT_EN_R {
        EF_IF_SBOOT_EN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:9"]
    #[inline(always)]
    pub fn ef_if_uart_dis(&self) -> EF_IF_UART_DIS_R {
        EF_IF_UART_DIS_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ef_if_ble2_dis(&self) -> EF_IF_BLE2_DIS_R {
        EF_IF_BLE2_DIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ef_if_m1542_dis(&self) -> EF_IF_M1542_DIS_R {
        EF_IF_M1542_DIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn ef_if_sf_key_re_sel(&self) -> EF_IF_SF_KEY_RE_SEL_R {
        EF_IF_SF_KEY_RE_SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ef_if_sdu_dis(&self) -> EF_IF_SDU_DIS_R {
        EF_IF_SDU_DIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ef_if_btdm_dis(&self) -> EF_IF_BTDM_DIS_R {
        EF_IF_BTDM_DIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ef_if_wifi_dis(&self) -> EF_IF_WIFI_DIS_R {
        EF_IF_WIFI_DIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ef_if_0_key_enc_en(&self) -> EF_IF_0_KEY_ENC_EN_R {
        EF_IF_0_KEY_ENC_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ef_if_cam_dis(&self) -> EF_IF_CAM_DIS_R {
        EF_IF_CAM_DIS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ef_if_m154_dis(&self) -> EF_IF_M154_DIS_R {
        EF_IF_M154_DIS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ef_if_cpu1_dis(&self) -> EF_IF_CPU1_DIS_R {
        EF_IF_CPU1_DIS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ef_if_cpu_rst_dbg_dis(&self) -> EF_IF_CPU_RST_DBG_DIS_R {
        EF_IF_CPU_RST_DBG_DIS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ef_if_se_dbg_dis(&self) -> EF_IF_SE_DBG_DIS_R {
        EF_IF_SE_DBG_DIS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ef_if_efuse_dbg_dis(&self) -> EF_IF_EFUSE_DBG_DIS_R {
        EF_IF_EFUSE_DBG_DIS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ef_if_dbg_jtag_1_dis(&self) -> EF_IF_DBG_JTAG_1_DIS_R {
        EF_IF_DBG_JTAG_1_DIS_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn ef_if_dbg_jtag_0_dis(&self) -> EF_IF_DBG_JTAG_0_DIS_R {
        EF_IF_DBG_JTAG_0_DIS_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn ef_if_dbg_mode(&self) -> EF_IF_DBG_MODE_R {
        EF_IF_DBG_MODE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ef_if_cfg_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_if_cfg_0](index.html) module"]
pub struct EF_IF_CFG_0_SPEC;
impl crate::RegisterSpec for EF_IF_CFG_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_if_cfg_0::R](R) reader structure"]
impl crate::Readable for EF_IF_CFG_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ef_if_cfg_0::W](W) writer structure"]
impl crate::Writable for EF_IF_CFG_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ef_if_cfg_0 to value 0"]
impl crate::Resettable for EF_IF_CFG_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
