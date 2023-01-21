#[doc = "Register `bmx_cfg2` reader"]
pub struct R(crate::R<BMX_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMX_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMX_CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMX_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `bmx_cfg2` writer"]
pub struct W(crate::W<BMX_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMX_CFG2_SPEC>;
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
impl From<crate::W<BMX_CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMX_CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_bmx_berr_en` reader - "]
pub type REG_BMX_BERR_EN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_bmx_berr_en` writer - "]
pub type REG_BMX_BERR_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BMX_CFG2_SPEC, u16, u16, 14, O>;
#[doc = "Field `reserved_14_15` reader - "]
pub type RESERVED_14_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_mcu_berr_en` reader - "]
pub type REG_MCU_BERR_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_mcu_berr_en` writer - "]
pub type REG_MCU_BERR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMX_CFG2_SPEC, bool, O>;
#[doc = "Field `reserved_17_31` reader - "]
pub type RESERVED_17_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn reg_bmx_berr_en(&self) -> REG_BMX_BERR_EN_R {
        REG_BMX_BERR_EN_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn reserved_14_15(&self) -> RESERVED_14_15_R {
        RESERVED_14_15_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_mcu_berr_en(&self) -> REG_MCU_BERR_EN_R {
        REG_MCU_BERR_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31"]
    #[inline(always)]
    pub fn reserved_17_31(&self) -> RESERVED_17_31_R {
        RESERVED_17_31_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_berr_en(&mut self) -> REG_BMX_BERR_EN_W<0> {
        REG_BMX_BERR_EN_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu_berr_en(&mut self) -> REG_MCU_BERR_EN_W<16> {
        REG_MCU_BERR_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "bmx_cfg2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmx_cfg2](index.html) module"]
pub struct BMX_CFG2_SPEC;
impl crate::RegisterSpec for BMX_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmx_cfg2::R](R) reader structure"]
impl crate::Readable for BMX_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmx_cfg2::W](W) writer structure"]
impl crate::Writable for BMX_CFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets bmx_cfg2 to value 0x0001_3fff"]
impl crate::Resettable for BMX_CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_3fff;
}
