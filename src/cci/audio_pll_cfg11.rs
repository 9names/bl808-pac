#[doc = "Register `audio_pll_cfg11` reader"]
pub struct R(crate::R<AUDIO_PLL_CFG11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUDIO_PLL_CFG11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUDIO_PLL_CFG11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUDIO_PLL_CFG11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `audio_pll_cfg11` writer"]
pub struct W(crate::W<AUDIO_PLL_CFG11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUDIO_PLL_CFG11_SPEC>;
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
impl From<crate::W<AUDIO_PLL_CFG11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUDIO_PLL_CFG11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `aupll_resv` reader - "]
pub type AUPLL_RESV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `aupll_resv` writer - "]
pub type AUPLL_RESV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AUDIO_PLL_CFG11_SPEC, u16, u16, 16, O>;
#[doc = "Field `reserved_16_22` reader - "]
pub type RESERVED_16_22_R = crate::FieldReader<u8, u8>;
#[doc = "Field `aupll_dl_ctrl_15` reader - "]
pub type AUPLL_DL_CTRL_15_R = crate::BitReader<bool>;
#[doc = "Field `aupll_dl_ctrl_15` writer - "]
pub type AUPLL_DL_CTRL_15_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIO_PLL_CFG11_SPEC, bool, O>;
#[doc = "Field `aupll_dl_ctrl_10` reader - "]
pub type AUPLL_DL_CTRL_10_R = crate::BitReader<bool>;
#[doc = "Field `aupll_dl_ctrl_10` writer - "]
pub type AUPLL_DL_CTRL_10_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIO_PLL_CFG11_SPEC, bool, O>;
#[doc = "Field `aupll_dl_ctrl_6` reader - "]
pub type AUPLL_DL_CTRL_6_R = crate::BitReader<bool>;
#[doc = "Field `aupll_dl_ctrl_6` writer - "]
pub type AUPLL_DL_CTRL_6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIO_PLL_CFG11_SPEC, bool, O>;
#[doc = "Field `aupll_dl_ctrl_5` reader - "]
pub type AUPLL_DL_CTRL_5_R = crate::BitReader<bool>;
#[doc = "Field `aupll_dl_ctrl_5` writer - "]
pub type AUPLL_DL_CTRL_5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIO_PLL_CFG11_SPEC, bool, O>;
#[doc = "Field `aupll_dl_ctrl_4` reader - "]
pub type AUPLL_DL_CTRL_4_R = crate::BitReader<bool>;
#[doc = "Field `aupll_dl_ctrl_4` writer - "]
pub type AUPLL_DL_CTRL_4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIO_PLL_CFG11_SPEC, bool, O>;
#[doc = "Field `aupll_dl_ctrl_3` reader - "]
pub type AUPLL_DL_CTRL_3_R = crate::BitReader<bool>;
#[doc = "Field `aupll_dl_ctrl_3` writer - "]
pub type AUPLL_DL_CTRL_3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIO_PLL_CFG11_SPEC, bool, O>;
#[doc = "Field `aupll_dl_ctrl_2p5` reader - "]
pub type AUPLL_DL_CTRL_2P5_R = crate::BitReader<bool>;
#[doc = "Field `aupll_dl_ctrl_2p5` writer - "]
pub type AUPLL_DL_CTRL_2P5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIO_PLL_CFG11_SPEC, bool, O>;
#[doc = "Field `aupll_dl_ctrl_2` reader - "]
pub type AUPLL_DL_CTRL_2_R = crate::BitReader<bool>;
#[doc = "Field `aupll_dl_ctrl_2` writer - "]
pub type AUPLL_DL_CTRL_2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIO_PLL_CFG11_SPEC, bool, O>;
#[doc = "Field `aupll_dl_ctrl_1` reader - "]
pub type AUPLL_DL_CTRL_1_R = crate::BitReader<bool>;
#[doc = "Field `aupll_dl_ctrl_1` writer - "]
pub type AUPLL_DL_CTRL_1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUDIO_PLL_CFG11_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn aupll_resv(&self) -> AUPLL_RESV_R {
        AUPLL_RESV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn reserved_16_22(&self) -> RESERVED_16_22_R {
        RESERVED_16_22_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_15(&self) -> AUPLL_DL_CTRL_15_R {
        AUPLL_DL_CTRL_15_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_10(&self) -> AUPLL_DL_CTRL_10_R {
        AUPLL_DL_CTRL_10_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_6(&self) -> AUPLL_DL_CTRL_6_R {
        AUPLL_DL_CTRL_6_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_5(&self) -> AUPLL_DL_CTRL_5_R {
        AUPLL_DL_CTRL_5_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_4(&self) -> AUPLL_DL_CTRL_4_R {
        AUPLL_DL_CTRL_4_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_3(&self) -> AUPLL_DL_CTRL_3_R {
        AUPLL_DL_CTRL_3_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_2p5(&self) -> AUPLL_DL_CTRL_2P5_R {
        AUPLL_DL_CTRL_2P5_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_2(&self) -> AUPLL_DL_CTRL_2_R {
        AUPLL_DL_CTRL_2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_1(&self) -> AUPLL_DL_CTRL_1_R {
        AUPLL_DL_CTRL_1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_resv(&mut self) -> AUPLL_RESV_W<0> {
        AUPLL_RESV_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_dl_ctrl_15(&mut self) -> AUPLL_DL_CTRL_15_W<23> {
        AUPLL_DL_CTRL_15_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_dl_ctrl_10(&mut self) -> AUPLL_DL_CTRL_10_W<24> {
        AUPLL_DL_CTRL_10_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_dl_ctrl_6(&mut self) -> AUPLL_DL_CTRL_6_W<25> {
        AUPLL_DL_CTRL_6_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_dl_ctrl_5(&mut self) -> AUPLL_DL_CTRL_5_W<26> {
        AUPLL_DL_CTRL_5_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_dl_ctrl_4(&mut self) -> AUPLL_DL_CTRL_4_W<27> {
        AUPLL_DL_CTRL_4_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_dl_ctrl_3(&mut self) -> AUPLL_DL_CTRL_3_W<28> {
        AUPLL_DL_CTRL_3_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_dl_ctrl_2p5(&mut self) -> AUPLL_DL_CTRL_2P5_W<29> {
        AUPLL_DL_CTRL_2P5_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_dl_ctrl_2(&mut self) -> AUPLL_DL_CTRL_2_W<30> {
        AUPLL_DL_CTRL_2_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_dl_ctrl_1(&mut self) -> AUPLL_DL_CTRL_1_W<31> {
        AUPLL_DL_CTRL_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "audio_pll_cfg11\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audio_pll_cfg11](index.html) module"]
pub struct AUDIO_PLL_CFG11_SPEC;
impl crate::RegisterSpec for AUDIO_PLL_CFG11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [audio_pll_cfg11::R](R) reader structure"]
impl crate::Readable for AUDIO_PLL_CFG11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [audio_pll_cfg11::W](W) writer structure"]
impl crate::Writable for AUDIO_PLL_CFG11_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets audio_pll_cfg11 to value 0"]
impl crate::Resettable for AUDIO_PLL_CFG11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
