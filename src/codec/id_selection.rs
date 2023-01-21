#[doc = "Register `id_selection` reader"]
pub struct R(crate::R<ID_SELECTION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID_SELECTION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID_SELECTION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID_SELECTION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `id_selection` writer"]
pub struct W(crate::W<ID_SELECTION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ID_SELECTION_SPEC>;
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
impl From<crate::W<ID_SELECTION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ID_SELECTION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rg_jenc_id_sel` reader - "]
pub type RG_JENC_ID_SEL_R = crate::BitReader<bool>;
#[doc = "Field `rg_jenc_id_sel` writer - "]
pub type RG_JENC_ID_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ID_SELECTION_SPEC, bool, O>;
#[doc = "Field `reserved_1_31` reader - "]
pub type RESERVED_1_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rg_jenc_id_sel(&self) -> RG_JENC_ID_SEL_R {
        RG_JENC_ID_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn reserved_1_31(&self) -> RESERVED_1_31_R {
        RESERVED_1_31_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rg_jenc_id_sel(&mut self) -> RG_JENC_ID_SEL_W<0> {
        RG_JENC_ID_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "id_selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_selection](index.html) module"]
pub struct ID_SELECTION_SPEC;
impl crate::RegisterSpec for ID_SELECTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id_selection::R](R) reader structure"]
impl crate::Readable for ID_SELECTION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [id_selection::W](W) writer structure"]
impl crate::Writable for ID_SELECTION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets id_selection to value 0"]
impl crate::Resettable for ID_SELECTION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
