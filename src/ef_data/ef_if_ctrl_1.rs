#[doc = "Register `ef_if_ctrl_1` reader"]
pub struct R(crate::R<EF_IF_CTRL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_IF_CTRL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EF_IF_CTRL_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EF_IF_CTRL_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ef_if_ctrl_1` writer"]
pub struct W(crate::W<EF_IF_CTRL_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EF_IF_CTRL_1_SPEC>;
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
impl From<crate::W<EF_IF_CTRL_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EF_IF_CTRL_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0_1` reader - "]
pub type RESERVED_0_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ef_if_1_busy` reader - "]
pub type EF_IF_1_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_1_rw` reader - "]
pub type EF_IF_1_RW_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_1_rw` writer - "]
pub type EF_IF_1_RW_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_IF_CTRL_1_SPEC, bool, O>;
#[doc = "Field `ef_if_1_trig` reader - "]
pub type EF_IF_1_TRIG_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_1_trig` writer - "]
pub type EF_IF_1_TRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_IF_CTRL_1_SPEC, bool, O>;
#[doc = "Field `ef_if_1_manual_en` reader - "]
pub type EF_IF_1_MANUAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_1_manual_en` writer - "]
pub type EF_IF_1_MANUAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_IF_CTRL_1_SPEC, bool, O>;
#[doc = "Field `ef_if_1_cyc_modify` reader - "]
pub type EF_IF_1_CYC_MODIFY_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_1_cyc_modify` writer - "]
pub type EF_IF_1_CYC_MODIFY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_IF_CTRL_1_SPEC, bool, O>;
#[doc = "Field `reserved_7_19` reader - "]
pub type RESERVED_7_19_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ef_if_1_int` reader - "]
pub type EF_IF_1_INT_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_1_int_clr` reader - "]
pub type EF_IF_1_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_1_int_clr` writer - "]
pub type EF_IF_1_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_IF_CTRL_1_SPEC, bool, O>;
#[doc = "Field `ef_if_1_int_set` reader - "]
pub type EF_IF_1_INT_SET_R = crate::BitReader<bool>;
#[doc = "Field `ef_if_1_int_set` writer - "]
pub type EF_IF_1_INT_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, EF_IF_CTRL_1_SPEC, bool, O>;
#[doc = "Field `reserved_23_31` reader - "]
pub type RESERVED_23_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn reserved_0_1(&self) -> RESERVED_0_1_R {
        RESERVED_0_1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ef_if_1_busy(&self) -> EF_IF_1_BUSY_R {
        EF_IF_1_BUSY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ef_if_1_rw(&self) -> EF_IF_1_RW_R {
        EF_IF_1_RW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ef_if_1_trig(&self) -> EF_IF_1_TRIG_R {
        EF_IF_1_TRIG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ef_if_1_manual_en(&self) -> EF_IF_1_MANUAL_EN_R {
        EF_IF_1_MANUAL_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ef_if_1_cyc_modify(&self) -> EF_IF_1_CYC_MODIFY_R {
        EF_IF_1_CYC_MODIFY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:19"]
    #[inline(always)]
    pub fn reserved_7_19(&self) -> RESERVED_7_19_R {
        RESERVED_7_19_R::new(((self.bits >> 7) & 0x1fff) as u16)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ef_if_1_int(&self) -> EF_IF_1_INT_R {
        EF_IF_1_INT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ef_if_1_int_clr(&self) -> EF_IF_1_INT_CLR_R {
        EF_IF_1_INT_CLR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ef_if_1_int_set(&self) -> EF_IF_1_INT_SET_R {
        EF_IF_1_INT_SET_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:31"]
    #[inline(always)]
    pub fn reserved_23_31(&self) -> RESERVED_23_31_R {
        RESERVED_23_31_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_1_rw(&mut self) -> EF_IF_1_RW_W<3> {
        EF_IF_1_RW_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_1_trig(&mut self) -> EF_IF_1_TRIG_W<4> {
        EF_IF_1_TRIG_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_1_manual_en(&mut self) -> EF_IF_1_MANUAL_EN_W<5> {
        EF_IF_1_MANUAL_EN_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_1_cyc_modify(&mut self) -> EF_IF_1_CYC_MODIFY_W<6> {
        EF_IF_1_CYC_MODIFY_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_1_int_clr(&mut self) -> EF_IF_1_INT_CLR_W<21> {
        EF_IF_1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn ef_if_1_int_set(&mut self) -> EF_IF_1_INT_SET_W<22> {
        EF_IF_1_INT_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ef_if_ctrl_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_if_ctrl_1](index.html) module"]
pub struct EF_IF_CTRL_1_SPEC;
impl crate::RegisterSpec for EF_IF_CTRL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_if_ctrl_1::R](R) reader structure"]
impl crate::Readable for EF_IF_CTRL_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ef_if_ctrl_1::W](W) writer structure"]
impl crate::Writable for EF_IF_CTRL_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ef_if_ctrl_1 to value 0x0020_0000"]
impl crate::Resettable for EF_IF_CTRL_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0020_0000;
}
