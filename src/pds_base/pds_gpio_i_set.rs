#[doc = "Register `pds_gpio_i_set` reader"]
pub struct R(crate::R<PDS_GPIO_I_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDS_GPIO_I_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDS_GPIO_I_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDS_GPIO_I_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pds_gpio_i_set` writer"]
pub struct W(crate::W<PDS_GPIO_I_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDS_GPIO_I_SET_SPEC>;
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
impl From<crate::W<PDS_GPIO_I_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDS_GPIO_I_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_pds_gpio_ie_set` reader - "]
pub type CR_PDS_GPIO_IE_SET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_pds_gpio_ie_set` writer - "]
pub type CR_PDS_GPIO_IE_SET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDS_GPIO_I_SET_SPEC, u8, u8, 3, O>;
#[doc = "Field `cr_pds_gpio_pd_set` reader - "]
pub type CR_PDS_GPIO_PD_SET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_pds_gpio_pd_set` writer - "]
pub type CR_PDS_GPIO_PD_SET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDS_GPIO_I_SET_SPEC, u8, u8, 3, O>;
#[doc = "Field `cr_pds_gpio_pu_set` reader - "]
pub type CR_PDS_GPIO_PU_SET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_pds_gpio_pu_set` writer - "]
pub type CR_PDS_GPIO_PU_SET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDS_GPIO_I_SET_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_9_31` reader - "]
pub type RESERVED_9_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn cr_pds_gpio_ie_set(&self) -> CR_PDS_GPIO_IE_SET_R {
        CR_PDS_GPIO_IE_SET_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn cr_pds_gpio_pd_set(&self) -> CR_PDS_GPIO_PD_SET_R {
        CR_PDS_GPIO_PD_SET_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn cr_pds_gpio_pu_set(&self) -> CR_PDS_GPIO_PU_SET_R {
        CR_PDS_GPIO_PU_SET_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:31"]
    #[inline(always)]
    pub fn reserved_9_31(&self) -> RESERVED_9_31_R {
        RESERVED_9_31_R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_gpio_ie_set(&mut self) -> CR_PDS_GPIO_IE_SET_W<0> {
        CR_PDS_GPIO_IE_SET_W::new(self)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_gpio_pd_set(&mut self) -> CR_PDS_GPIO_PD_SET_W<3> {
        CR_PDS_GPIO_PD_SET_W::new(self)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_gpio_pu_set(&mut self) -> CR_PDS_GPIO_PU_SET_W<6> {
        CR_PDS_GPIO_PU_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pds_gpio_i_set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_gpio_i_set](index.html) module"]
pub struct PDS_GPIO_I_SET_SPEC;
impl crate::RegisterSpec for PDS_GPIO_I_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pds_gpio_i_set::R](R) reader structure"]
impl crate::Readable for PDS_GPIO_I_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pds_gpio_i_set::W](W) writer structure"]
impl crate::Writable for PDS_GPIO_I_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pds_gpio_i_set to value 0"]
impl crate::Resettable for PDS_GPIO_I_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
