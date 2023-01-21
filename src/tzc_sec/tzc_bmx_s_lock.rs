#[doc = "Register `tzc_bmx_s_lock` reader"]
pub struct R(crate::R<TZC_BMX_S_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_BMX_S_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_BMX_S_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_BMX_S_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_bmx_s_lock` writer"]
pub struct W(crate::W<TZC_BMX_S_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_BMX_S_LOCK_SPEC>;
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
impl From<crate::W<TZC_BMX_S_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_BMX_S_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_bmx_s10_tzsid_lock` reader - "]
pub type TZC_BMX_S10_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s10_tzsid_lock` writer - "]
pub type TZC_BMX_S10_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s11_tzsid_lock` reader - "]
pub type TZC_BMX_S11_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s11_tzsid_lock` writer - "]
pub type TZC_BMX_S11_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s12_tzsid_lock` reader - "]
pub type TZC_BMX_S12_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s12_tzsid_lock` writer - "]
pub type TZC_BMX_S12_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s13_tzsid_lock` reader - "]
pub type TZC_BMX_S13_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s13_tzsid_lock` writer - "]
pub type TZC_BMX_S13_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s14_tzsid_lock` reader - "]
pub type TZC_BMX_S14_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s14_tzsid_lock` writer - "]
pub type TZC_BMX_S14_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s15_tzsid_lock` reader - "]
pub type TZC_BMX_S15_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s15_tzsid_lock` writer - "]
pub type TZC_BMX_S15_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s16_tzsid_lock` reader - "]
pub type TZC_BMX_S16_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s16_tzsid_lock` writer - "]
pub type TZC_BMX_S16_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s17_tzsid_lock` reader - "]
pub type TZC_BMX_S17_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s17_tzsid_lock` writer - "]
pub type TZC_BMX_S17_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s18_tzsid_lock` reader - "]
pub type TZC_BMX_S18_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s18_tzsid_lock` writer - "]
pub type TZC_BMX_S18_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s19_tzsid_lock` reader - "]
pub type TZC_BMX_S19_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s19_tzsid_lock` writer - "]
pub type TZC_BMX_S19_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s1a_tzsid_lock` reader - "]
pub type TZC_BMX_S1A_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s1a_tzsid_lock` writer - "]
pub type TZC_BMX_S1A_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s1b_tzsid_lock` reader - "]
pub type TZC_BMX_S1B_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s1b_tzsid_lock` writer - "]
pub type TZC_BMX_S1B_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s1c_tzsid_lock` reader - "]
pub type TZC_BMX_S1C_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s1c_tzsid_lock` writer - "]
pub type TZC_BMX_S1C_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s1d_tzsid_lock` reader - "]
pub type TZC_BMX_S1D_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s1d_tzsid_lock` writer - "]
pub type TZC_BMX_S1D_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s1e_tzsid_lock` reader - "]
pub type TZC_BMX_S1E_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s1e_tzsid_lock` writer - "]
pub type TZC_BMX_S1E_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s1f_tzsid_lock` reader - "]
pub type TZC_BMX_S1F_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s1f_tzsid_lock` writer - "]
pub type TZC_BMX_S1F_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s20_tzsid_lock` reader - "]
pub type TZC_BMX_S20_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s20_tzsid_lock` writer - "]
pub type TZC_BMX_S20_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s21_tzsid_lock` reader - "]
pub type TZC_BMX_S21_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s21_tzsid_lock` writer - "]
pub type TZC_BMX_S21_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s22_tzsid_lock` reader - "]
pub type TZC_BMX_S22_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s22_tzsid_lock` writer - "]
pub type TZC_BMX_S22_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s23_tzsid_lock` reader - "]
pub type TZC_BMX_S23_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s23_tzsid_lock` writer - "]
pub type TZC_BMX_S23_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s24_tzsid_lock` reader - "]
pub type TZC_BMX_S24_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s24_tzsid_lock` writer - "]
pub type TZC_BMX_S24_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s25_tzsid_lock` reader - "]
pub type TZC_BMX_S25_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s25_tzsid_lock` writer - "]
pub type TZC_BMX_S25_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s26_tzsid_lock` reader - "]
pub type TZC_BMX_S26_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s26_tzsid_lock` writer - "]
pub type TZC_BMX_S26_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s27_tzsid_lock` reader - "]
pub type TZC_BMX_S27_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s27_tzsid_lock` writer - "]
pub type TZC_BMX_S27_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s28_tzsid_lock` reader - "]
pub type TZC_BMX_S28_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s28_tzsid_lock` writer - "]
pub type TZC_BMX_S28_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s29_tzsid_lock` reader - "]
pub type TZC_BMX_S29_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s29_tzsid_lock` writer - "]
pub type TZC_BMX_S29_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s2a_tzsid_lock` reader - "]
pub type TZC_BMX_S2A_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s2a_tzsid_lock` writer - "]
pub type TZC_BMX_S2A_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s2b_tzsid_lock` reader - "]
pub type TZC_BMX_S2B_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s2b_tzsid_lock` writer - "]
pub type TZC_BMX_S2B_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s2c_tzsid_lock` reader - "]
pub type TZC_BMX_S2C_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s2c_tzsid_lock` writer - "]
pub type TZC_BMX_S2C_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s2d_tzsid_lock` reader - "]
pub type TZC_BMX_S2D_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s2d_tzsid_lock` writer - "]
pub type TZC_BMX_S2D_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s2e_tzsid_lock` reader - "]
pub type TZC_BMX_S2E_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s2e_tzsid_lock` writer - "]
pub type TZC_BMX_S2E_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
#[doc = "Field `tzc_bmx_s2f_tzsid_lock` reader - "]
pub type TZC_BMX_S2F_TZSID_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_bmx_s2f_tzsid_lock` writer - "]
pub type TZC_BMX_S2F_TZSID_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_BMX_S_LOCK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_bmx_s10_tzsid_lock(&self) -> TZC_BMX_S10_TZSID_LOCK_R {
        TZC_BMX_S10_TZSID_LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_bmx_s11_tzsid_lock(&self) -> TZC_BMX_S11_TZSID_LOCK_R {
        TZC_BMX_S11_TZSID_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_bmx_s12_tzsid_lock(&self) -> TZC_BMX_S12_TZSID_LOCK_R {
        TZC_BMX_S12_TZSID_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_bmx_s13_tzsid_lock(&self) -> TZC_BMX_S13_TZSID_LOCK_R {
        TZC_BMX_S13_TZSID_LOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tzc_bmx_s14_tzsid_lock(&self) -> TZC_BMX_S14_TZSID_LOCK_R {
        TZC_BMX_S14_TZSID_LOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tzc_bmx_s15_tzsid_lock(&self) -> TZC_BMX_S15_TZSID_LOCK_R {
        TZC_BMX_S15_TZSID_LOCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tzc_bmx_s16_tzsid_lock(&self) -> TZC_BMX_S16_TZSID_LOCK_R {
        TZC_BMX_S16_TZSID_LOCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tzc_bmx_s17_tzsid_lock(&self) -> TZC_BMX_S17_TZSID_LOCK_R {
        TZC_BMX_S17_TZSID_LOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tzc_bmx_s18_tzsid_lock(&self) -> TZC_BMX_S18_TZSID_LOCK_R {
        TZC_BMX_S18_TZSID_LOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tzc_bmx_s19_tzsid_lock(&self) -> TZC_BMX_S19_TZSID_LOCK_R {
        TZC_BMX_S19_TZSID_LOCK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tzc_bmx_s1a_tzsid_lock(&self) -> TZC_BMX_S1A_TZSID_LOCK_R {
        TZC_BMX_S1A_TZSID_LOCK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tzc_bmx_s1b_tzsid_lock(&self) -> TZC_BMX_S1B_TZSID_LOCK_R {
        TZC_BMX_S1B_TZSID_LOCK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tzc_bmx_s1c_tzsid_lock(&self) -> TZC_BMX_S1C_TZSID_LOCK_R {
        TZC_BMX_S1C_TZSID_LOCK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tzc_bmx_s1d_tzsid_lock(&self) -> TZC_BMX_S1D_TZSID_LOCK_R {
        TZC_BMX_S1D_TZSID_LOCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tzc_bmx_s1e_tzsid_lock(&self) -> TZC_BMX_S1E_TZSID_LOCK_R {
        TZC_BMX_S1E_TZSID_LOCK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tzc_bmx_s1f_tzsid_lock(&self) -> TZC_BMX_S1F_TZSID_LOCK_R {
        TZC_BMX_S1F_TZSID_LOCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_bmx_s20_tzsid_lock(&self) -> TZC_BMX_S20_TZSID_LOCK_R {
        TZC_BMX_S20_TZSID_LOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_bmx_s21_tzsid_lock(&self) -> TZC_BMX_S21_TZSID_LOCK_R {
        TZC_BMX_S21_TZSID_LOCK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_bmx_s22_tzsid_lock(&self) -> TZC_BMX_S22_TZSID_LOCK_R {
        TZC_BMX_S22_TZSID_LOCK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tzc_bmx_s23_tzsid_lock(&self) -> TZC_BMX_S23_TZSID_LOCK_R {
        TZC_BMX_S23_TZSID_LOCK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tzc_bmx_s24_tzsid_lock(&self) -> TZC_BMX_S24_TZSID_LOCK_R {
        TZC_BMX_S24_TZSID_LOCK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tzc_bmx_s25_tzsid_lock(&self) -> TZC_BMX_S25_TZSID_LOCK_R {
        TZC_BMX_S25_TZSID_LOCK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tzc_bmx_s26_tzsid_lock(&self) -> TZC_BMX_S26_TZSID_LOCK_R {
        TZC_BMX_S26_TZSID_LOCK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tzc_bmx_s27_tzsid_lock(&self) -> TZC_BMX_S27_TZSID_LOCK_R {
        TZC_BMX_S27_TZSID_LOCK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_bmx_s28_tzsid_lock(&self) -> TZC_BMX_S28_TZSID_LOCK_R {
        TZC_BMX_S28_TZSID_LOCK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_bmx_s29_tzsid_lock(&self) -> TZC_BMX_S29_TZSID_LOCK_R {
        TZC_BMX_S29_TZSID_LOCK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_bmx_s2a_tzsid_lock(&self) -> TZC_BMX_S2A_TZSID_LOCK_R {
        TZC_BMX_S2A_TZSID_LOCK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tzc_bmx_s2b_tzsid_lock(&self) -> TZC_BMX_S2B_TZSID_LOCK_R {
        TZC_BMX_S2B_TZSID_LOCK_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn tzc_bmx_s2c_tzsid_lock(&self) -> TZC_BMX_S2C_TZSID_LOCK_R {
        TZC_BMX_S2C_TZSID_LOCK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn tzc_bmx_s2d_tzsid_lock(&self) -> TZC_BMX_S2D_TZSID_LOCK_R {
        TZC_BMX_S2D_TZSID_LOCK_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn tzc_bmx_s2e_tzsid_lock(&self) -> TZC_BMX_S2E_TZSID_LOCK_R {
        TZC_BMX_S2E_TZSID_LOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tzc_bmx_s2f_tzsid_lock(&self) -> TZC_BMX_S2F_TZSID_LOCK_R {
        TZC_BMX_S2F_TZSID_LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s10_tzsid_lock(&mut self) -> TZC_BMX_S10_TZSID_LOCK_W<0> {
        TZC_BMX_S10_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s11_tzsid_lock(&mut self) -> TZC_BMX_S11_TZSID_LOCK_W<1> {
        TZC_BMX_S11_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s12_tzsid_lock(&mut self) -> TZC_BMX_S12_TZSID_LOCK_W<2> {
        TZC_BMX_S12_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s13_tzsid_lock(&mut self) -> TZC_BMX_S13_TZSID_LOCK_W<3> {
        TZC_BMX_S13_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s14_tzsid_lock(&mut self) -> TZC_BMX_S14_TZSID_LOCK_W<4> {
        TZC_BMX_S14_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s15_tzsid_lock(&mut self) -> TZC_BMX_S15_TZSID_LOCK_W<5> {
        TZC_BMX_S15_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s16_tzsid_lock(&mut self) -> TZC_BMX_S16_TZSID_LOCK_W<6> {
        TZC_BMX_S16_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s17_tzsid_lock(&mut self) -> TZC_BMX_S17_TZSID_LOCK_W<7> {
        TZC_BMX_S17_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s18_tzsid_lock(&mut self) -> TZC_BMX_S18_TZSID_LOCK_W<8> {
        TZC_BMX_S18_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s19_tzsid_lock(&mut self) -> TZC_BMX_S19_TZSID_LOCK_W<9> {
        TZC_BMX_S19_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s1a_tzsid_lock(&mut self) -> TZC_BMX_S1A_TZSID_LOCK_W<10> {
        TZC_BMX_S1A_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s1b_tzsid_lock(&mut self) -> TZC_BMX_S1B_TZSID_LOCK_W<11> {
        TZC_BMX_S1B_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s1c_tzsid_lock(&mut self) -> TZC_BMX_S1C_TZSID_LOCK_W<12> {
        TZC_BMX_S1C_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s1d_tzsid_lock(&mut self) -> TZC_BMX_S1D_TZSID_LOCK_W<13> {
        TZC_BMX_S1D_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s1e_tzsid_lock(&mut self) -> TZC_BMX_S1E_TZSID_LOCK_W<14> {
        TZC_BMX_S1E_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s1f_tzsid_lock(&mut self) -> TZC_BMX_S1F_TZSID_LOCK_W<15> {
        TZC_BMX_S1F_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s20_tzsid_lock(&mut self) -> TZC_BMX_S20_TZSID_LOCK_W<16> {
        TZC_BMX_S20_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s21_tzsid_lock(&mut self) -> TZC_BMX_S21_TZSID_LOCK_W<17> {
        TZC_BMX_S21_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s22_tzsid_lock(&mut self) -> TZC_BMX_S22_TZSID_LOCK_W<18> {
        TZC_BMX_S22_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s23_tzsid_lock(&mut self) -> TZC_BMX_S23_TZSID_LOCK_W<19> {
        TZC_BMX_S23_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s24_tzsid_lock(&mut self) -> TZC_BMX_S24_TZSID_LOCK_W<20> {
        TZC_BMX_S24_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s25_tzsid_lock(&mut self) -> TZC_BMX_S25_TZSID_LOCK_W<21> {
        TZC_BMX_S25_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s26_tzsid_lock(&mut self) -> TZC_BMX_S26_TZSID_LOCK_W<22> {
        TZC_BMX_S26_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s27_tzsid_lock(&mut self) -> TZC_BMX_S27_TZSID_LOCK_W<23> {
        TZC_BMX_S27_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s28_tzsid_lock(&mut self) -> TZC_BMX_S28_TZSID_LOCK_W<24> {
        TZC_BMX_S28_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s29_tzsid_lock(&mut self) -> TZC_BMX_S29_TZSID_LOCK_W<25> {
        TZC_BMX_S29_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s2a_tzsid_lock(&mut self) -> TZC_BMX_S2A_TZSID_LOCK_W<26> {
        TZC_BMX_S2A_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s2b_tzsid_lock(&mut self) -> TZC_BMX_S2B_TZSID_LOCK_W<27> {
        TZC_BMX_S2B_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s2c_tzsid_lock(&mut self) -> TZC_BMX_S2C_TZSID_LOCK_W<28> {
        TZC_BMX_S2C_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s2d_tzsid_lock(&mut self) -> TZC_BMX_S2D_TZSID_LOCK_W<29> {
        TZC_BMX_S2D_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s2e_tzsid_lock(&mut self) -> TZC_BMX_S2E_TZSID_LOCK_W<30> {
        TZC_BMX_S2E_TZSID_LOCK_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_s2f_tzsid_lock(&mut self) -> TZC_BMX_S2F_TZSID_LOCK_W<31> {
        TZC_BMX_S2F_TZSID_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tzc_bmx_s_lock\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_bmx_s_lock](index.html) module"]
pub struct TZC_BMX_S_LOCK_SPEC;
impl crate::RegisterSpec for TZC_BMX_S_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_bmx_s_lock::R](R) reader structure"]
impl crate::Readable for TZC_BMX_S_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_bmx_s_lock::W](W) writer structure"]
impl crate::Writable for TZC_BMX_S_LOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_bmx_s_lock to value 0"]
impl crate::Resettable for TZC_BMX_S_LOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
