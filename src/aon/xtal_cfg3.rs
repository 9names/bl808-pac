#[doc = "Register `xtal_cfg3` reader"]
pub struct R(crate::R<XTAL_CFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTAL_CFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTAL_CFG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTAL_CFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `xtal_cfg3` writer"]
pub struct W(crate::W<XTAL_CFG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTAL_CFG3_SPEC>;
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
impl From<crate::W<XTAL_CFG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTAL_CFG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0_11` reader - "]
pub type RESERVED_0_11_R = crate::FieldReader<u16, u16>;
#[doc = "Field `wifi_xtal_clk_inv_en_aon` reader - "]
pub type WIFI_XTAL_CLK_INV_EN_AON_R = crate::BitReader<bool>;
#[doc = "Field `wifi_xtal_clk_inv_en_aon` writer - "]
pub type WIFI_XTAL_CLK_INV_EN_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, XTAL_CFG3_SPEC, bool, O>;
#[doc = "Field `reserved_13_15` reader - "]
pub type RESERVED_13_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wifi_xtal_cml_en_aon` reader - "]
pub type WIFI_XTAL_CML_EN_AON_R = crate::BitReader<bool>;
#[doc = "Field `wifi_xtal_cml_en_aon` writer - "]
pub type WIFI_XTAL_CML_EN_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, XTAL_CFG3_SPEC, bool, O>;
#[doc = "Field `wifi_xtal_cml_r_sel_aon` reader - "]
pub type WIFI_XTAL_CML_R_SEL_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wifi_xtal_cml_r_sel_aon` writer - "]
pub type WIFI_XTAL_CML_R_SEL_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XTAL_CFG3_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_19` reader - "]
pub type RESERVED_19_R = crate::BitReader<bool>;
#[doc = "Field `wifi_xtal_clk_en_aon` reader - "]
pub type WIFI_XTAL_CLK_EN_AON_R = crate::BitReader<bool>;
#[doc = "Field `wifi_xtal_clk_en_aon` writer - "]
pub type WIFI_XTAL_CLK_EN_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, XTAL_CFG3_SPEC, bool, O>;
#[doc = "Field `reserved_21_29` reader - "]
pub type RESERVED_21_29_R = crate::FieldReader<u16, u16>;
#[doc = "Field `wifi_xtal_buf_drv_aon` reader - "]
pub type WIFI_XTAL_BUF_DRV_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wifi_xtal_buf_drv_aon` writer - "]
pub type WIFI_XTAL_BUF_DRV_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XTAL_CFG3_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn reserved_0_11(&self) -> RESERVED_0_11_R {
        RESERVED_0_11_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn wifi_xtal_clk_inv_en_aon(&self) -> WIFI_XTAL_CLK_INV_EN_AON_R {
        WIFI_XTAL_CLK_INV_EN_AON_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn reserved_13_15(&self) -> RESERVED_13_15_R {
        RESERVED_13_15_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn wifi_xtal_cml_en_aon(&self) -> WIFI_XTAL_CML_EN_AON_R {
        WIFI_XTAL_CML_EN_AON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn wifi_xtal_cml_r_sel_aon(&self) -> WIFI_XTAL_CML_R_SEL_AON_R {
        WIFI_XTAL_CML_R_SEL_AON_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reserved_19(&self) -> RESERVED_19_R {
        RESERVED_19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn wifi_xtal_clk_en_aon(&self) -> WIFI_XTAL_CLK_EN_AON_R {
        WIFI_XTAL_CLK_EN_AON_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:29"]
    #[inline(always)]
    pub fn reserved_21_29(&self) -> RESERVED_21_29_R {
        RESERVED_21_29_R::new(((self.bits >> 21) & 0x01ff) as u16)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn wifi_xtal_buf_drv_aon(&self) -> WIFI_XTAL_BUF_DRV_AON_R {
        WIFI_XTAL_BUF_DRV_AON_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_xtal_clk_inv_en_aon(&mut self) -> WIFI_XTAL_CLK_INV_EN_AON_W<12> {
        WIFI_XTAL_CLK_INV_EN_AON_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_xtal_cml_en_aon(&mut self) -> WIFI_XTAL_CML_EN_AON_W<16> {
        WIFI_XTAL_CML_EN_AON_W::new(self)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_xtal_cml_r_sel_aon(&mut self) -> WIFI_XTAL_CML_R_SEL_AON_W<17> {
        WIFI_XTAL_CML_R_SEL_AON_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_xtal_clk_en_aon(&mut self) -> WIFI_XTAL_CLK_EN_AON_W<20> {
        WIFI_XTAL_CLK_EN_AON_W::new(self)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_xtal_buf_drv_aon(&mut self) -> WIFI_XTAL_BUF_DRV_AON_W<30> {
        WIFI_XTAL_BUF_DRV_AON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "xtal_cfg3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal_cfg3](index.html) module"]
pub struct XTAL_CFG3_SPEC;
impl crate::RegisterSpec for XTAL_CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtal_cfg3::R](R) reader structure"]
impl crate::Readable for XTAL_CFG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtal_cfg3::W](W) writer structure"]
impl crate::Writable for XTAL_CFG3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets xtal_cfg3 to value 0x4012_0000"]
impl crate::Resettable for XTAL_CFG3_SPEC {
    const RESET_VALUE: Self::Ux = 0x4012_0000;
}
