#[doc = "Register `hbn_pad_ctrl_1` reader"]
pub struct R(crate::R<HBN_PAD_CTRL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HBN_PAD_CTRL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HBN_PAD_CTRL_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HBN_PAD_CTRL_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hbn_pad_ctrl_1` writer"]
pub struct W(crate::W<HBN_PAD_CTRL_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HBN_PAD_CTRL_1_SPEC>;
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
impl From<crate::W<HBN_PAD_CTRL_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HBN_PAD_CTRL_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_aon_pad_oe` reader - "]
pub type REG_AON_PAD_OE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_aon_pad_oe` writer - "]
pub type REG_AON_PAD_OE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HBN_PAD_CTRL_1_SPEC, u16, u16, 9, O>;
#[doc = "Field `reserved_9` reader - "]
pub type RESERVED_9_R = crate::BitReader<bool>;
#[doc = "Field `reg_aon_pad_pd` reader - "]
pub type REG_AON_PAD_PD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_aon_pad_pd` writer - "]
pub type REG_AON_PAD_PD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HBN_PAD_CTRL_1_SPEC, u16, u16, 9, O>;
#[doc = "Field `reserved_19` reader - "]
pub type RESERVED_19_R = crate::BitReader<bool>;
#[doc = "Field `reg_aon_pad_pu` reader - "]
pub type REG_AON_PAD_PU_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_aon_pad_pu` writer - "]
pub type REG_AON_PAD_PU_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HBN_PAD_CTRL_1_SPEC, u16, u16, 9, O>;
#[doc = "Field `reserved_29_31` reader - "]
pub type RESERVED_29_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn reg_aon_pad_oe(&self) -> REG_AON_PAD_OE_R {
        REG_AON_PAD_OE_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reserved_9(&self) -> RESERVED_9_R {
        RESERVED_9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:18"]
    #[inline(always)]
    pub fn reg_aon_pad_pd(&self) -> REG_AON_PAD_PD_R {
        REG_AON_PAD_PD_R::new(((self.bits >> 10) & 0x01ff) as u16)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reserved_19(&self) -> RESERVED_19_R {
        RESERVED_19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:28"]
    #[inline(always)]
    pub fn reg_aon_pad_pu(&self) -> REG_AON_PAD_PU_R {
        REG_AON_PAD_PU_R::new(((self.bits >> 20) & 0x01ff) as u16)
    }
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn reserved_29_31(&self) -> RESERVED_29_31_R {
        RESERVED_29_31_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    #[must_use]
    pub fn reg_aon_pad_oe(&mut self) -> REG_AON_PAD_OE_W<0> {
        REG_AON_PAD_OE_W::new(self)
    }
    #[doc = "Bits 10:18"]
    #[inline(always)]
    #[must_use]
    pub fn reg_aon_pad_pd(&mut self) -> REG_AON_PAD_PD_W<10> {
        REG_AON_PAD_PD_W::new(self)
    }
    #[doc = "Bits 20:28"]
    #[inline(always)]
    #[must_use]
    pub fn reg_aon_pad_pu(&mut self) -> REG_AON_PAD_PU_W<20> {
        REG_AON_PAD_PU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HBN_PAD_CTRL_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_pad_ctrl_1](index.html) module"]
pub struct HBN_PAD_CTRL_1_SPEC;
impl crate::RegisterSpec for HBN_PAD_CTRL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hbn_pad_ctrl_1::R](R) reader structure"]
impl crate::Readable for HBN_PAD_CTRL_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hbn_pad_ctrl_1::W](W) writer structure"]
impl crate::Writable for HBN_PAD_CTRL_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hbn_pad_ctrl_1 to value 0"]
impl crate::Resettable for HBN_PAD_CTRL_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
