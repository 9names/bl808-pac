#[doc = "Register `swrst_cfg3` reader"]
pub struct R(crate::R<SWRST_CFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWRST_CFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWRST_CFG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWRST_CFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `swrst_cfg3` writer"]
pub struct W(crate::W<SWRST_CFG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWRST_CFG3_SPEC>;
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
impl From<crate::W<SWRST_CFG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWRST_CFG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0_1` reader - "]
pub type RESERVED_0_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `disrst_s12` reader - "]
pub type DISRST_S12_R = crate::BitReader<bool>;
#[doc = "Field `disrst_s12` writer - "]
pub type DISRST_S12_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG3_SPEC, bool, O>;
#[doc = "Field `reserved_3` reader - "]
pub type RESERVED_3_R = crate::BitReader<bool>;
#[doc = "Field `disrst_s14` reader - "]
pub type DISRST_S14_R = crate::BitReader<bool>;
#[doc = "Field `disrst_s14` writer - "]
pub type DISRST_S14_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG3_SPEC, bool, O>;
#[doc = "Field `reserved_5_7` reader - "]
pub type RESERVED_5_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `disrst_s18` reader - "]
pub type DISRST_S18_R = crate::BitReader<bool>;
#[doc = "Field `disrst_s18` writer - "]
pub type DISRST_S18_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG3_SPEC, bool, O>;
#[doc = "Field `reserved_9_10` reader - "]
pub type RESERVED_9_10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `disrst_s1b` reader - "]
pub type DISRST_S1B_R = crate::BitReader<bool>;
#[doc = "Field `disrst_s1b` writer - "]
pub type DISRST_S1B_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG3_SPEC, bool, O>;
#[doc = "Field `reserved_12_15` reader - "]
pub type RESERVED_12_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `disrst_s1a0` reader - "]
pub type DISRST_S1A0_R = crate::BitReader<bool>;
#[doc = "Field `disrst_s1a0` writer - "]
pub type DISRST_S1A0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG3_SPEC, bool, O>;
#[doc = "Field `disrst_s1a1` reader - "]
pub type DISRST_S1A1_R = crate::BitReader<bool>;
#[doc = "Field `disrst_s1a1` writer - "]
pub type DISRST_S1A1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG3_SPEC, bool, O>;
#[doc = "Field `disrst_s1a2` reader - "]
pub type DISRST_S1A2_R = crate::BitReader<bool>;
#[doc = "Field `disrst_s1a2` writer - "]
pub type DISRST_S1A2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG3_SPEC, bool, O>;
#[doc = "Field `disrst_s1a3` reader - "]
pub type DISRST_S1A3_R = crate::BitReader<bool>;
#[doc = "Field `disrst_s1a3` writer - "]
pub type DISRST_S1A3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG3_SPEC, bool, O>;
#[doc = "Field `disrst_s1a4` reader - "]
pub type DISRST_S1A4_R = crate::BitReader<bool>;
#[doc = "Field `disrst_s1a4` writer - "]
pub type DISRST_S1A4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG3_SPEC, bool, O>;
#[doc = "Field `disrst_s1a5` reader - "]
pub type DISRST_S1A5_R = crate::BitReader<bool>;
#[doc = "Field `disrst_s1a5` writer - "]
pub type DISRST_S1A5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG3_SPEC, bool, O>;
#[doc = "Field `disrst_s1a6` reader - "]
pub type DISRST_S1A6_R = crate::BitReader<bool>;
#[doc = "Field `disrst_s1a6` writer - "]
pub type DISRST_S1A6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG3_SPEC, bool, O>;
#[doc = "Field `disrst_s1a7` reader - "]
pub type DISRST_S1A7_R = crate::BitReader<bool>;
#[doc = "Field `disrst_s1a7` writer - "]
pub type DISRST_S1A7_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG3_SPEC, bool, O>;
#[doc = "Field `disrst_s1a8` reader - "]
pub type DISRST_S1A8_R = crate::BitReader<bool>;
#[doc = "Field `disrst_s1a8` writer - "]
pub type DISRST_S1A8_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG3_SPEC, bool, O>;
#[doc = "Field `disrst_s1a9` reader - "]
pub type DISRST_S1A9_R = crate::BitReader<bool>;
#[doc = "Field `disrst_s1a9` writer - "]
pub type DISRST_S1A9_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG3_SPEC, bool, O>;
#[doc = "Field `disrst_s1aa` reader - "]
pub type DISRST_S1AA_R = crate::BitReader<bool>;
#[doc = "Field `disrst_s1aa` writer - "]
pub type DISRST_S1AA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG3_SPEC, bool, O>;
#[doc = "Field `reserved_27_31` reader - "]
pub type RESERVED_27_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn reserved_0_1(&self) -> RESERVED_0_1_R {
        RESERVED_0_1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn disrst_s12(&self) -> DISRST_S12_R {
        DISRST_S12_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reserved_3(&self) -> RESERVED_3_R {
        RESERVED_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn disrst_s14(&self) -> DISRST_S14_R {
        DISRST_S14_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn reserved_5_7(&self) -> RESERVED_5_7_R {
        RESERVED_5_7_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn disrst_s18(&self) -> DISRST_S18_R {
        DISRST_S18_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn reserved_9_10(&self) -> RESERVED_9_10_R {
        RESERVED_9_10_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn disrst_s1b(&self) -> DISRST_S1B_R {
        DISRST_S1B_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn reserved_12_15(&self) -> RESERVED_12_15_R {
        RESERVED_12_15_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn disrst_s1a0(&self) -> DISRST_S1A0_R {
        DISRST_S1A0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn disrst_s1a1(&self) -> DISRST_S1A1_R {
        DISRST_S1A1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn disrst_s1a2(&self) -> DISRST_S1A2_R {
        DISRST_S1A2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn disrst_s1a3(&self) -> DISRST_S1A3_R {
        DISRST_S1A3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn disrst_s1a4(&self) -> DISRST_S1A4_R {
        DISRST_S1A4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn disrst_s1a5(&self) -> DISRST_S1A5_R {
        DISRST_S1A5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn disrst_s1a6(&self) -> DISRST_S1A6_R {
        DISRST_S1A6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn disrst_s1a7(&self) -> DISRST_S1A7_R {
        DISRST_S1A7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn disrst_s1a8(&self) -> DISRST_S1A8_R {
        DISRST_S1A8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn disrst_s1a9(&self) -> DISRST_S1A9_R {
        DISRST_S1A9_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn disrst_s1aa(&self) -> DISRST_S1AA_R {
        DISRST_S1AA_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn reserved_27_31(&self) -> RESERVED_27_31_R {
        RESERVED_27_31_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn disrst_s12(&mut self) -> DISRST_S12_W<2> {
        DISRST_S12_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn disrst_s14(&mut self) -> DISRST_S14_W<4> {
        DISRST_S14_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn disrst_s18(&mut self) -> DISRST_S18_W<8> {
        DISRST_S18_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn disrst_s1b(&mut self) -> DISRST_S1B_W<11> {
        DISRST_S1B_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn disrst_s1a0(&mut self) -> DISRST_S1A0_W<16> {
        DISRST_S1A0_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn disrst_s1a1(&mut self) -> DISRST_S1A1_W<17> {
        DISRST_S1A1_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn disrst_s1a2(&mut self) -> DISRST_S1A2_W<18> {
        DISRST_S1A2_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn disrst_s1a3(&mut self) -> DISRST_S1A3_W<19> {
        DISRST_S1A3_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn disrst_s1a4(&mut self) -> DISRST_S1A4_W<20> {
        DISRST_S1A4_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn disrst_s1a5(&mut self) -> DISRST_S1A5_W<21> {
        DISRST_S1A5_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn disrst_s1a6(&mut self) -> DISRST_S1A6_W<22> {
        DISRST_S1A6_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn disrst_s1a7(&mut self) -> DISRST_S1A7_W<23> {
        DISRST_S1A7_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn disrst_s1a8(&mut self) -> DISRST_S1A8_W<24> {
        DISRST_S1A8_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn disrst_s1a9(&mut self) -> DISRST_S1A9_W<25> {
        DISRST_S1A9_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn disrst_s1aa(&mut self) -> DISRST_S1AA_W<26> {
        DISRST_S1AA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Disable hreset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swrst_cfg3](index.html) module"]
pub struct SWRST_CFG3_SPEC;
impl crate::RegisterSpec for SWRST_CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swrst_cfg3::R](R) reader structure"]
impl crate::Readable for SWRST_CFG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swrst_cfg3::W](W) writer structure"]
impl crate::Writable for SWRST_CFG3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets swrst_cfg3 to value 0"]
impl crate::Resettable for SWRST_CFG3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
