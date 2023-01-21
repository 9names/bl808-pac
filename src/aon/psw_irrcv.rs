#[doc = "Register `psw_irrcv` reader"]
pub struct R(crate::R<PSW_IRRCV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSW_IRRCV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSW_IRRCV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSW_IRRCV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `psw_irrcv` writer"]
pub struct W(crate::W<PSW_IRRCV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSW_IRRCV_SPEC>;
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
impl From<crate::W<PSW_IRRCV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSW_IRRCV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pu_psw_irrcv_aon` reader - "]
pub type PU_PSW_IRRCV_AON_R = crate::BitReader<bool>;
#[doc = "Field `pu_psw_irrcv_aon` writer - "]
pub type PU_PSW_IRRCV_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSW_IRRCV_SPEC, bool, O>;
#[doc = "Field `reserved_1_18` reader - "]
pub type RESERVED_1_18_R = crate::FieldReader<u32, u32>;
#[doc = "Field `usb20_rref_ext_en_aon` reader - "]
pub type USB20_RREF_EXT_EN_AON_R = crate::BitReader<bool>;
#[doc = "Field `usb20_rref_ext_en_aon` writer - "]
pub type USB20_RREF_EXT_EN_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PSW_IRRCV_SPEC, bool, O>;
#[doc = "Field `en_por33_aon` reader - "]
pub type EN_POR33_AON_R = crate::BitReader<bool>;
#[doc = "Field `en_por33_aon` writer - "]
pub type EN_POR33_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSW_IRRCV_SPEC, bool, O>;
#[doc = "Field `usb20_rref_hiz_aon` reader - "]
pub type USB20_RREF_HIZ_AON_R = crate::BitReader<bool>;
#[doc = "Field `usb20_rref_hiz_aon` writer - "]
pub type USB20_RREF_HIZ_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSW_IRRCV_SPEC, bool, O>;
#[doc = "Field `reserved_22_23` reader - "]
pub type RESERVED_22_23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `usb20_rcal_code_aon` reader - "]
pub type USB20_RCAL_CODE_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `usb20_rcal_code_aon` writer - "]
pub type USB20_RCAL_CODE_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSW_IRRCV_SPEC, u8, u8, 6, O>;
#[doc = "Field `reserved_30_31` reader - "]
pub type RESERVED_30_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_psw_irrcv_aon(&self) -> PU_PSW_IRRCV_AON_R {
        PU_PSW_IRRCV_AON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:18"]
    #[inline(always)]
    pub fn reserved_1_18(&self) -> RESERVED_1_18_R {
        RESERVED_1_18_R::new((self.bits >> 1) & 0x0003_ffff)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn usb20_rref_ext_en_aon(&self) -> USB20_RREF_EXT_EN_AON_R {
        USB20_RREF_EXT_EN_AON_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn en_por33_aon(&self) -> EN_POR33_AON_R {
        EN_POR33_AON_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn usb20_rref_hiz_aon(&self) -> USB20_RREF_HIZ_AON_R {
        USB20_RREF_HIZ_AON_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn reserved_22_23(&self) -> RESERVED_22_23_R {
        RESERVED_22_23_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn usb20_rcal_code_aon(&self) -> USB20_RCAL_CODE_AON_R {
        USB20_RCAL_CODE_AON_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reserved_30_31(&self) -> RESERVED_30_31_R {
        RESERVED_30_31_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pu_psw_irrcv_aon(&mut self) -> PU_PSW_IRRCV_AON_W<0> {
        PU_PSW_IRRCV_AON_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn usb20_rref_ext_en_aon(&mut self) -> USB20_RREF_EXT_EN_AON_W<19> {
        USB20_RREF_EXT_EN_AON_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn en_por33_aon(&mut self) -> EN_POR33_AON_W<20> {
        EN_POR33_AON_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn usb20_rref_hiz_aon(&mut self) -> USB20_RREF_HIZ_AON_W<21> {
        USB20_RREF_HIZ_AON_W::new(self)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    #[must_use]
    pub fn usb20_rcal_code_aon(&mut self) -> USB20_RCAL_CODE_AON_W<24> {
        USB20_RCAL_CODE_AON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "psw_irrcv\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psw_irrcv](index.html) module"]
pub struct PSW_IRRCV_SPEC;
impl crate::RegisterSpec for PSW_IRRCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psw_irrcv::R](R) reader structure"]
impl crate::Readable for PSW_IRRCV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psw_irrcv::W](W) writer structure"]
impl crate::Writable for PSW_IRRCV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets psw_irrcv to value 0x1a00_0000"]
impl crate::Resettable for PSW_IRRCV_SPEC {
    const RESET_VALUE: Self::Ux = 0x1a00_0000;
}
