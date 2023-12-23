#[doc = "Register `pds_gpio_stat` reader"]
pub struct R(crate::R<PDS_GPIO_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDS_GPIO_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDS_GPIO_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDS_GPIO_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pds_gpio_stat` writer"]
pub struct W(crate::W<PDS_GPIO_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDS_GPIO_STAT_SPEC>;
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
impl From<crate::W<PDS_GPIO_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDS_GPIO_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pds_gpio_int_stat` reader - "]
pub type PDS_GPIO_INT_STAT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pds_gpio_int_stat(&self) -> PDS_GPIO_INT_STAT_R {
        PDS_GPIO_INT_STAT_R::new(self.bits)
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
#[doc = "pds_gpio_stat\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_gpio_stat](index.html) module"]
pub struct PDS_GPIO_STAT_SPEC;
impl crate::RegisterSpec for PDS_GPIO_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pds_gpio_stat::R](R) reader structure"]
impl crate::Readable for PDS_GPIO_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pds_gpio_stat::W](W) writer structure"]
impl crate::Writable for PDS_GPIO_STAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pds_gpio_stat to value 0"]
impl crate::Resettable for PDS_GPIO_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
