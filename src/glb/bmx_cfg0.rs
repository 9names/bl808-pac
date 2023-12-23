#[doc = "Register `bmx_cfg0` reader"]
pub struct R(crate::R<BMX_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMX_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMX_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMX_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `bmx_cfg0` writer"]
pub struct W(crate::W<BMX_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMX_CFG0_SPEC>;
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
impl From<crate::W<BMX_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMX_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_bmx_timeout_en` reader - "]
pub type REG_BMX_TIMEOUT_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_bmx_timeout_en` writer - "]
pub type REG_BMX_TIMEOUT_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BMX_CFG0_SPEC, u8, u8, 5, O>;
#[doc = "Field `reg_bmx_arb_mode` reader - "]
pub type REG_BMX_ARB_MODE_R = crate::BitReader<bool>;
#[doc = "Field `reg_bmx_arb_mode` writer - "]
pub type REG_BMX_ARB_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMX_CFG0_SPEC, bool, O>;
#[doc = "Field `reg_bmx_timeout_clr` reader - "]
pub type REG_BMX_TIMEOUT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `reg_bmx_timeout_clr` writer - "]
pub type REG_BMX_TIMEOUT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMX_CFG0_SPEC, bool, O>;
#[doc = "Field `reg_h_wthre_hw2ext` reader - "]
pub type REG_H_WTHRE_HW2EXT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_h_wthre_hw2ext` writer - "]
pub type REG_H_WTHRE_HW2EXT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BMX_CFG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `bmx_busy_option_dis` reader - "]
pub type BMX_BUSY_OPTION_DIS_R = crate::BitReader<bool>;
#[doc = "Field `bmx_busy_option_dis` writer - "]
pub type BMX_BUSY_OPTION_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMX_CFG0_SPEC, bool, O>;
#[doc = "Field `bmx_gating_dis` reader - "]
pub type BMX_GATING_DIS_R = crate::BitReader<bool>;
#[doc = "Field `bmx_gating_dis` writer - "]
pub type BMX_GATING_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMX_CFG0_SPEC, bool, O>;
#[doc = "Field `sts_bmx_timeout_sts` reader - "]
pub type STS_BMX_TIMEOUT_STS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pds_apb_cfg` reader - "]
pub type PDS_APB_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pds_apb_cfg` writer - "]
pub type PDS_APB_CFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BMX_CFG0_SPEC, u8, u8, 8, O>;
#[doc = "Field `hbn_apb_cfg` reader - "]
pub type HBN_APB_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `hbn_apb_cfg` writer - "]
pub type HBN_APB_CFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BMX_CFG0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn reg_bmx_timeout_en(&self) -> REG_BMX_TIMEOUT_EN_R {
        REG_BMX_TIMEOUT_EN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_bmx_arb_mode(&self) -> REG_BMX_ARB_MODE_R {
        REG_BMX_ARB_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_bmx_timeout_clr(&self) -> REG_BMX_TIMEOUT_CLR_R {
        REG_BMX_TIMEOUT_CLR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn reg_h_wthre_hw2ext(&self) -> REG_H_WTHRE_HW2EXT_R {
        REG_H_WTHRE_HW2EXT_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bmx_busy_option_dis(&self) -> BMX_BUSY_OPTION_DIS_R {
        BMX_BUSY_OPTION_DIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn bmx_gating_dis(&self) -> BMX_GATING_DIS_R {
        BMX_GATING_DIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn sts_bmx_timeout_sts(&self) -> STS_BMX_TIMEOUT_STS_R {
        STS_BMX_TIMEOUT_STS_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn pds_apb_cfg(&self) -> PDS_APB_CFG_R {
        PDS_APB_CFG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn hbn_apb_cfg(&self) -> HBN_APB_CFG_R {
        HBN_APB_CFG_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_timeout_en(&mut self) -> REG_BMX_TIMEOUT_EN_W<0> {
        REG_BMX_TIMEOUT_EN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_arb_mode(&mut self) -> REG_BMX_ARB_MODE_W<5> {
        REG_BMX_ARB_MODE_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_timeout_clr(&mut self) -> REG_BMX_TIMEOUT_CLR_W<6> {
        REG_BMX_TIMEOUT_CLR_W::new(self)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    #[must_use]
    pub fn reg_h_wthre_hw2ext(&mut self) -> REG_H_WTHRE_HW2EXT_W<7> {
        REG_H_WTHRE_HW2EXT_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn bmx_busy_option_dis(&mut self) -> BMX_BUSY_OPTION_DIS_W<9> {
        BMX_BUSY_OPTION_DIS_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn bmx_gating_dis(&mut self) -> BMX_GATING_DIS_W<10> {
        BMX_GATING_DIS_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn pds_apb_cfg(&mut self) -> PDS_APB_CFG_W<16> {
        PDS_APB_CFG_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_apb_cfg(&mut self) -> HBN_APB_CFG_W<24> {
        HBN_APB_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "bmx_cfg0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmx_cfg0](index.html) module"]
pub struct BMX_CFG0_SPEC;
impl crate::RegisterSpec for BMX_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmx_cfg0::R](R) reader structure"]
impl crate::Readable for BMX_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmx_cfg0::W](W) writer structure"]
impl crate::Writable for BMX_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets bmx_cfg0 to value 0"]
impl crate::Resettable for BMX_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
