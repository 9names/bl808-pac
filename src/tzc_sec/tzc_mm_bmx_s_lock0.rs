#[doc = "Register `tzc_mm_bmx_s_lock0` reader"]
pub struct R(crate::R<TZC_MM_BMX_S_LOCK0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_MM_BMX_S_LOCK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_MM_BMX_S_LOCK0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_MM_BMX_S_LOCK0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_mm_bmx_s_lock0` writer"]
pub struct W(crate::W<TZC_MM_BMX_S_LOCK0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_MM_BMX_S_LOCK0_SPEC>;
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
impl From<crate::W<TZC_MM_BMX_S_LOCK0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_MM_BMX_S_LOCK0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_mmperi_s10_tzsid_lock` reader - "]
pub type TZC_MMPERI_S10_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_mmperi_s10_tzsid_lock` writer - "]
pub type TZC_MMPERI_S10_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_MM_BMX_S_LOCK0_SPEC, bool, O>;
#[doc = "Field `tzc_mmperi_s11_tzsid_lock` reader - "]
pub type TZC_MMPERI_S11_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_mmperi_s11_tzsid_lock` writer - "]
pub type TZC_MMPERI_S11_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_MM_BMX_S_LOCK0_SPEC, bool, O>;
#[doc = "Field `tzc_mmperi_s12_tzsid_lock` reader - "]
pub type TZC_MMPERI_S12_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_mmperi_s12_tzsid_lock` writer - "]
pub type TZC_MMPERI_S12_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_MM_BMX_S_LOCK0_SPEC, bool, O>;
#[doc = "Field `tzc_mmperi_s13_tzsid_lock` reader - "]
pub type TZC_MMPERI_S13_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_mmperi_s13_tzsid_lock` writer - "]
pub type TZC_MMPERI_S13_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_MM_BMX_S_LOCK0_SPEC, bool, O>;
#[doc = "Field `tzc_mmperi_s14_tzsid_lock` reader - "]
pub type TZC_MMPERI_S14_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_mmperi_s14_tzsid_lock` writer - "]
pub type TZC_MMPERI_S14_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_MM_BMX_S_LOCK0_SPEC, bool, O>;
#[doc = "Field `tzc_mmperi_s15_tzsid_lock` reader - "]
pub type TZC_MMPERI_S15_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_mmperi_s15_tzsid_lock` writer - "]
pub type TZC_MMPERI_S15_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_MM_BMX_S_LOCK0_SPEC, bool, O>;
#[doc = "Field `tzc_mmperi_s16_tzsid_lock` reader - "]
pub type TZC_MMPERI_S16_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_mmperi_s16_tzsid_lock` writer - "]
pub type TZC_MMPERI_S16_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_MM_BMX_S_LOCK0_SPEC, bool, O>;
#[doc = "Field `tzc_mmperi_s17_tzsid_lock` reader - "]
pub type TZC_MMPERI_S17_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_mmperi_s17_tzsid_lock` writer - "]
pub type TZC_MMPERI_S17_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_MM_BMX_S_LOCK0_SPEC, bool, O>;
#[doc = "Field `tzc_mmperi_s18_tzsid_lock` reader - "]
pub type TZC_MMPERI_S18_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_mmperi_s18_tzsid_lock` writer - "]
pub type TZC_MMPERI_S18_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_MM_BMX_S_LOCK0_SPEC, bool, O>;
#[doc = "Field `tzc_mmperi_s19_tzsid_lock` reader - "]
pub type TZC_MMPERI_S19_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_mmperi_s19_tzsid_lock` writer - "]
pub type TZC_MMPERI_S19_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_MM_BMX_S_LOCK0_SPEC, bool, O>;
#[doc = "Field `tzc_mmperi_s1a_tzsid_lock` reader - "]
pub type TZC_MMPERI_S1A_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_mmperi_s1a_tzsid_lock` writer - "]
pub type TZC_MMPERI_S1A_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_MM_BMX_S_LOCK0_SPEC, bool, O>;
#[doc = "Field `tzc_mmperi_s1b_tzsid_lock` reader - "]
pub type TZC_MMPERI_S1B_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_mmperi_s1b_tzsid_lock` writer - "]
pub type TZC_MMPERI_S1B_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_MM_BMX_S_LOCK0_SPEC, bool, O>;
#[doc = "Field `tzc_mmperi_s1c_tzsid_lock` reader - "]
pub type TZC_MMPERI_S1C_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_mmperi_s1c_tzsid_lock` writer - "]
pub type TZC_MMPERI_S1C_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_MM_BMX_S_LOCK0_SPEC, bool, O>;
#[doc = "Field `tzc_mmperi_s1d_tzsid_lock` reader - "]
pub type TZC_MMPERI_S1D_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_mmperi_s1d_tzsid_lock` writer - "]
pub type TZC_MMPERI_S1D_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_MM_BMX_S_LOCK0_SPEC, bool, O>;
#[doc = "Field `tzc_mmperi_s1e_tzsid_lock` reader - "]
pub type TZC_MMPERI_S1E_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_mmperi_s1e_tzsid_lock` writer - "]
pub type TZC_MMPERI_S1E_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_MM_BMX_S_LOCK0_SPEC, bool, O>;
#[doc = "Field `tzc_mmperi_s1f_tzsid_lock` reader - "]
pub type TZC_MMPERI_S1F_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_mmperi_s1f_tzsid_lock` writer - "]
pub type TZC_MMPERI_S1F_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_MM_BMX_S_LOCK0_SPEC, bool, O>;
#[doc = "Field `reserved_16_31` reader - "]
pub type RESERVED_16_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_mmperi_s10_tzsid_lock(&self) -> TZC_MMPERI_S10_TZSID_LOCK_R {
        TZC_MMPERI_S10_TZSID_LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_mmperi_s11_tzsid_lock(&self) -> TZC_MMPERI_S11_TZSID_LOCK_R {
        TZC_MMPERI_S11_TZSID_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_mmperi_s12_tzsid_lock(&self) -> TZC_MMPERI_S12_TZSID_LOCK_R {
        TZC_MMPERI_S12_TZSID_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_mmperi_s13_tzsid_lock(&self) -> TZC_MMPERI_S13_TZSID_LOCK_R {
        TZC_MMPERI_S13_TZSID_LOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tzc_mmperi_s14_tzsid_lock(&self) -> TZC_MMPERI_S14_TZSID_LOCK_R {
        TZC_MMPERI_S14_TZSID_LOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tzc_mmperi_s15_tzsid_lock(&self) -> TZC_MMPERI_S15_TZSID_LOCK_R {
        TZC_MMPERI_S15_TZSID_LOCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tzc_mmperi_s16_tzsid_lock(&self) -> TZC_MMPERI_S16_TZSID_LOCK_R {
        TZC_MMPERI_S16_TZSID_LOCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tzc_mmperi_s17_tzsid_lock(&self) -> TZC_MMPERI_S17_TZSID_LOCK_R {
        TZC_MMPERI_S17_TZSID_LOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tzc_mmperi_s18_tzsid_lock(&self) -> TZC_MMPERI_S18_TZSID_LOCK_R {
        TZC_MMPERI_S18_TZSID_LOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tzc_mmperi_s19_tzsid_lock(&self) -> TZC_MMPERI_S19_TZSID_LOCK_R {
        TZC_MMPERI_S19_TZSID_LOCK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tzc_mmperi_s1a_tzsid_lock(&self) -> TZC_MMPERI_S1A_TZSID_LOCK_R {
        TZC_MMPERI_S1A_TZSID_LOCK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tzc_mmperi_s1b_tzsid_lock(&self) -> TZC_MMPERI_S1B_TZSID_LOCK_R {
        TZC_MMPERI_S1B_TZSID_LOCK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tzc_mmperi_s1c_tzsid_lock(&self) -> TZC_MMPERI_S1C_TZSID_LOCK_R {
        TZC_MMPERI_S1C_TZSID_LOCK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tzc_mmperi_s1d_tzsid_lock(&self) -> TZC_MMPERI_S1D_TZSID_LOCK_R {
        TZC_MMPERI_S1D_TZSID_LOCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tzc_mmperi_s1e_tzsid_lock(&self) -> TZC_MMPERI_S1E_TZSID_LOCK_R {
        TZC_MMPERI_S1E_TZSID_LOCK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tzc_mmperi_s1f_tzsid_lock(&self) -> TZC_MMPERI_S1F_TZSID_LOCK_R {
        TZC_MMPERI_S1F_TZSID_LOCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn reserved_16_31(&self) -> RESERVED_16_31_R {
        RESERVED_16_31_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_mmperi_s10_tzsid_lock(&mut self) -> TZC_MMPERI_S10_TZSID_LOCK_W<0> {
        TZC_MMPERI_S10_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_mmperi_s11_tzsid_lock(&mut self) -> TZC_MMPERI_S11_TZSID_LOCK_W<1> {
        TZC_MMPERI_S11_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_mmperi_s12_tzsid_lock(&mut self) -> TZC_MMPERI_S12_TZSID_LOCK_W<2> {
        TZC_MMPERI_S12_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_mmperi_s13_tzsid_lock(&mut self) -> TZC_MMPERI_S13_TZSID_LOCK_W<3> {
        TZC_MMPERI_S13_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_mmperi_s14_tzsid_lock(&mut self) -> TZC_MMPERI_S14_TZSID_LOCK_W<4> {
        TZC_MMPERI_S14_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_mmperi_s15_tzsid_lock(&mut self) -> TZC_MMPERI_S15_TZSID_LOCK_W<5> {
        TZC_MMPERI_S15_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_mmperi_s16_tzsid_lock(&mut self) -> TZC_MMPERI_S16_TZSID_LOCK_W<6> {
        TZC_MMPERI_S16_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_mmperi_s17_tzsid_lock(&mut self) -> TZC_MMPERI_S17_TZSID_LOCK_W<7> {
        TZC_MMPERI_S17_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_mmperi_s18_tzsid_lock(&mut self) -> TZC_MMPERI_S18_TZSID_LOCK_W<8> {
        TZC_MMPERI_S18_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_mmperi_s19_tzsid_lock(&mut self) -> TZC_MMPERI_S19_TZSID_LOCK_W<9> {
        TZC_MMPERI_S19_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_mmperi_s1a_tzsid_lock(&mut self) -> TZC_MMPERI_S1A_TZSID_LOCK_W<10> {
        TZC_MMPERI_S1A_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_mmperi_s1b_tzsid_lock(&mut self) -> TZC_MMPERI_S1B_TZSID_LOCK_W<11> {
        TZC_MMPERI_S1B_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_mmperi_s1c_tzsid_lock(&mut self) -> TZC_MMPERI_S1C_TZSID_LOCK_W<12> {
        TZC_MMPERI_S1C_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_mmperi_s1d_tzsid_lock(&mut self) -> TZC_MMPERI_S1D_TZSID_LOCK_W<13> {
        TZC_MMPERI_S1D_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_mmperi_s1e_tzsid_lock(&mut self) -> TZC_MMPERI_S1E_TZSID_LOCK_W<14> {
        TZC_MMPERI_S1E_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_mmperi_s1f_tzsid_lock(&mut self) -> TZC_MMPERI_S1F_TZSID_LOCK_W<15> {
        TZC_MMPERI_S1F_TZSID_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tzc_mm_bmx_s_lock0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_mm_bmx_s_lock0](index.html) module"]
pub struct TZC_MM_BMX_S_LOCK0_SPEC;
impl crate::RegisterSpec for TZC_MM_BMX_S_LOCK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_mm_bmx_s_lock0::R](R) reader structure"]
impl crate::Readable for TZC_MM_BMX_S_LOCK0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_mm_bmx_s_lock0::W](W) writer structure"]
impl crate::Writable for TZC_MM_BMX_S_LOCK0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_mm_bmx_s_lock0 to value 0"]
impl crate::Resettable for TZC_MM_BMX_S_LOCK0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}