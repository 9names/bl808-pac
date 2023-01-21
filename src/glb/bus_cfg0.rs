#[doc = "Register `bus_cfg0` reader"]
pub struct R(crate::R<BUS_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUS_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUS_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUS_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `bus_cfg0` writer"]
pub struct W(crate::W<BUS_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUS_CFG0_SPEC>;
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
impl From<crate::W<BUS_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUS_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rg_apb2_pck_force` reader - "]
pub type RG_APB2_PCK_FORCE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rg_apb2_pck_force` writer - "]
pub type RG_APB2_PCK_FORCE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BUS_CFG0_SPEC, u16, u16, 16, O>;
#[doc = "Field `reserved_16_31` reader - "]
pub type RESERVED_16_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rg_apb2_pck_force(&self) -> RG_APB2_PCK_FORCE_R {
        RG_APB2_PCK_FORCE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn reserved_16_31(&self) -> RESERVED_16_31_R {
        RESERVED_16_31_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn rg_apb2_pck_force(&mut self) -> RG_APB2_PCK_FORCE_W<0> {
        RG_APB2_PCK_FORCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "bus_cfg0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_cfg0](index.html) module"]
pub struct BUS_CFG0_SPEC;
impl crate::RegisterSpec for BUS_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bus_cfg0::R](R) reader structure"]
impl crate::Readable for BUS_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bus_cfg0::W](W) writer structure"]
impl crate::Writable for BUS_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets bus_cfg0 to value 0xffff"]
impl crate::Resettable for BUS_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
