#[doc = "Register `xtal_cfg2` reader"]
pub struct R(crate::R<XTAL_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTAL_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTAL_CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTAL_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `xtal_cfg2` writer"]
pub struct W(crate::W<XTAL_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTAL_CFG2_SPEC>;
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
impl From<crate::W<XTAL_CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTAL_CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wifi_xtal_ldo33_bypass_aon` reader - "]
pub type WIFI_XTAL_LDO33_BYPASS_AON_R = crate::BitReader<bool>;
#[doc = "Field `wifi_xtal_ldo33_bypass_aon` writer - "]
pub type WIFI_XTAL_LDO33_BYPASS_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, XTAL_CFG2_SPEC, bool, O>;
#[doc = "Field `wifi_xtal_ldo33_sel_aon` reader - "]
pub type WIFI_XTAL_LDO33_SEL_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wifi_xtal_ldo33_sel_aon` writer - "]
pub type WIFI_XTAL_LDO33_SEL_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XTAL_CFG2_SPEC, u8, u8, 3, O>;
#[doc = "Field `wifi_xtal_ldo18_sel_aon` reader - "]
pub type WIFI_XTAL_LDO18_SEL_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wifi_xtal_ldo18_sel_aon` writer - "]
pub type WIFI_XTAL_LDO18_SEL_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XTAL_CFG2_SPEC, u8, u8, 2, O>;
#[doc = "Field `wifi_xtal_ldo33_pu_aon` reader - "]
pub type WIFI_XTAL_LDO33_PU_AON_R = crate::BitReader<bool>;
#[doc = "Field `wifi_xtal_ldo33_pu_aon` writer - "]
pub type WIFI_XTAL_LDO33_PU_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, XTAL_CFG2_SPEC, bool, O>;
#[doc = "Field `wifi_xtal_ldo18_pu_aon` reader - "]
pub type WIFI_XTAL_LDO18_PU_AON_R = crate::BitReader<bool>;
#[doc = "Field `wifi_xtal_ldo18_pu_aon` writer - "]
pub type WIFI_XTAL_LDO18_PU_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, XTAL_CFG2_SPEC, bool, O>;
#[doc = "Field `reserved_8_9` reader - "]
pub type RESERVED_8_9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wifi_xtal_reserve` reader - "]
pub type WIFI_XTAL_RESERVE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wifi_xtal_reserve` writer - "]
pub type WIFI_XTAL_RESERVE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XTAL_CFG2_SPEC, u8, u8, 4, O>;
#[doc = "Field `reserved_14_15` reader - "]
pub type RESERVED_14_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wifi_xtal_ldo18_short_filter_aon` reader - "]
pub type WIFI_XTAL_LDO18_SHORT_FILTER_AON_R = crate::BitReader<bool>;
#[doc = "Field `wifi_xtal_ldo18_short_filter_aon` writer - "]
pub type WIFI_XTAL_LDO18_SHORT_FILTER_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, XTAL_CFG2_SPEC, bool, O>;
#[doc = "Field `reserved_17_29` reader - "]
pub type RESERVED_17_29_R = crate::FieldReader<u16, u16>;
#[doc = "Field `xtal_buf_drv_aon` reader - "]
pub type XTAL_BUF_DRV_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `xtal_buf_drv_aon` writer - "]
pub type XTAL_BUF_DRV_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XTAL_CFG2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wifi_xtal_ldo33_bypass_aon(&self) -> WIFI_XTAL_LDO33_BYPASS_AON_R {
        WIFI_XTAL_LDO33_BYPASS_AON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn wifi_xtal_ldo33_sel_aon(&self) -> WIFI_XTAL_LDO33_SEL_AON_R {
        WIFI_XTAL_LDO33_SEL_AON_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn wifi_xtal_ldo18_sel_aon(&self) -> WIFI_XTAL_LDO18_SEL_AON_R {
        WIFI_XTAL_LDO18_SEL_AON_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn wifi_xtal_ldo33_pu_aon(&self) -> WIFI_XTAL_LDO33_PU_AON_R {
        WIFI_XTAL_LDO33_PU_AON_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn wifi_xtal_ldo18_pu_aon(&self) -> WIFI_XTAL_LDO18_PU_AON_R {
        WIFI_XTAL_LDO18_PU_AON_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn reserved_8_9(&self) -> RESERVED_8_9_R {
        RESERVED_8_9_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:13"]
    #[inline(always)]
    pub fn wifi_xtal_reserve(&self) -> WIFI_XTAL_RESERVE_R {
        WIFI_XTAL_RESERVE_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn reserved_14_15(&self) -> RESERVED_14_15_R {
        RESERVED_14_15_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn wifi_xtal_ldo18_short_filter_aon(&self) -> WIFI_XTAL_LDO18_SHORT_FILTER_AON_R {
        WIFI_XTAL_LDO18_SHORT_FILTER_AON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:29"]
    #[inline(always)]
    pub fn reserved_17_29(&self) -> RESERVED_17_29_R {
        RESERVED_17_29_R::new(((self.bits >> 17) & 0x1fff) as u16)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn xtal_buf_drv_aon(&self) -> XTAL_BUF_DRV_AON_R {
        XTAL_BUF_DRV_AON_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_xtal_ldo33_bypass_aon(&mut self) -> WIFI_XTAL_LDO33_BYPASS_AON_W<0> {
        WIFI_XTAL_LDO33_BYPASS_AON_W::new(self)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_xtal_ldo33_sel_aon(&mut self) -> WIFI_XTAL_LDO33_SEL_AON_W<1> {
        WIFI_XTAL_LDO33_SEL_AON_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_xtal_ldo18_sel_aon(&mut self) -> WIFI_XTAL_LDO18_SEL_AON_W<4> {
        WIFI_XTAL_LDO18_SEL_AON_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_xtal_ldo33_pu_aon(&mut self) -> WIFI_XTAL_LDO33_PU_AON_W<6> {
        WIFI_XTAL_LDO33_PU_AON_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_xtal_ldo18_pu_aon(&mut self) -> WIFI_XTAL_LDO18_PU_AON_W<7> {
        WIFI_XTAL_LDO18_PU_AON_W::new(self)
    }
    #[doc = "Bits 10:13"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_xtal_reserve(&mut self) -> WIFI_XTAL_RESERVE_W<10> {
        WIFI_XTAL_RESERVE_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_xtal_ldo18_short_filter_aon(&mut self) -> WIFI_XTAL_LDO18_SHORT_FILTER_AON_W<16> {
        WIFI_XTAL_LDO18_SHORT_FILTER_AON_W::new(self)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_buf_drv_aon(&mut self) -> XTAL_BUF_DRV_AON_W<30> {
        XTAL_BUF_DRV_AON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "xtal_cfg2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal_cfg2](index.html) module"]
pub struct XTAL_CFG2_SPEC;
impl crate::RegisterSpec for XTAL_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtal_cfg2::R](R) reader structure"]
impl crate::Readable for XTAL_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtal_cfg2::W](W) writer structure"]
impl crate::Writable for XTAL_CFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets xtal_cfg2 to value 0x4000_00d0"]
impl crate::Resettable for XTAL_CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000_00d0;
}
